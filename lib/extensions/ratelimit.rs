use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Header, Response, Server};

static mut LIMITER: Option<RateLimiter> = None;
static mut REQ_LIMIT: Option<u64> = None;
static mut REQ_TIMEOUT: Option<u64> = None;

/// Limit the amount of requests handled by the server.
pub struct RateLimiter {
    /// Requests Per Req_Timeout
    req_limit: u64,

    /// Time of last reset
    last_reset: u64,

    /// How often to reset the counters
    req_timeout: u64,

    /// Table of requests per IP
    requests: HashMap<String, u64>,
}

impl RateLimiter {
    /// Make a new RateLimiter.
    fn new(req_limit: u64, req_timeout: u64) -> RateLimiter {
        RateLimiter {
            req_limit,
            last_reset: 0,
            req_timeout,
            requests: HashMap::new(),
        }
    }

    /// Count a request.
    fn add_request(&mut self, ip: String) {
        self.requests
            .insert(ip.clone(), self.requests.get(&ip).unwrap_or(&0) + 1);
    }

    /// Check if request table needs to be cleared.
    fn check_reset(&mut self) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if self.last_reset + self.req_timeout <= time {
            self.requests = HashMap::new();
            self.last_reset = time;
        }
    }

    /// Check if the request limit has been reached for an ip.
    fn is_over_limit(&self, ip: String) -> bool {
        self.requests.get(&ip).unwrap_or(&0) >= &self.req_limit
    }

    /// Attach the rate limiter to a server.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use afire::{Server, RateLimiter};
    ///
    /// // Create a new server
    /// let mut server: Server = Server::new("localhost", 1234);
    ///
    /// // Enable Rate Limiting
    /// // This will limit the number of requests per IP to 5 per 10 seconds
    /// RateLimiter::attach(&mut server, 5, 10);
    ///
    /// // Start Server
    /// // This is blocking
    /// # server.set_run(false);
    /// server.start();
    /// ```
    pub fn attach(server: &mut Server, req_limit: u64, req_timeout: u64) {
        unsafe {
            REQ_LIMIT = Some(req_limit);
            REQ_TIMEOUT = Some(req_timeout);
        }

        server.every(Box::new(|req| {
            let ip = req
                .address
                .clone()
                .split(':')
                .collect::<Vec<&str>>()
                .first()
                .unwrap_or(&"null")
                .to_string();

            unsafe {
                // Init Limiter if not already
                if LIMITER.is_none() {
                    LIMITER = Some(RateLimiter::new(REQ_LIMIT.unwrap(), REQ_TIMEOUT.unwrap()));
                }

                // Check if we need to reset the limiter
                LIMITER.as_mut().unwrap().check_reset();

                if LIMITER.as_mut().unwrap().is_over_limit(ip.clone()) {
                    return Some(Response::new(
                        429,
                        "Too Many Requests",
                        vec![Header::new("Content-Type", "text/plain")],
                    ));
                }

                // Add Current request
                LIMITER.as_mut().unwrap().add_request(ip);
            }
            None
        }));
    }
}

// Allow printing of RateLimiter for debugging
impl fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RateLimiter")
            .field("req_limit", &self.req_limit)
            .field("req_timeout", &self.req_timeout)
            .field("last_reset", &self.last_reset)
            .field("requests", &self.requests)
            .finish()
    }
}
