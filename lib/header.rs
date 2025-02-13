use std::fmt;

/// Http header
///
/// Has a name and a value.
pub struct Header {
    pub(super) name: String,
    pub(super) value: String,
}

impl Header {
    /// Make a new header
    /// ## Example
    /// ```rust
    /// // Import Modules
    /// use afire::Header;
    ///
    /// let header1 = Header::new("Content-Type", "text/html");
    /// let header2 = Header::new("Access-Control-Allow-Origin", "*");
    /// ```
    pub fn new(name: &str, value: &str) -> Header {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Convert a header ref to a header
    /// ## Example
    /// ```rust
    /// // Import Modules
    /// use afire::Header;
    ///
    /// let header1 = &Header::new("Content-Type", "text/html");
    /// let header2 = Header::new("Content-Type", "text/html");
    ///
    /// assert!(header1.copy() == header2);
    /// ```
    pub fn copy(&self) -> Header {
        Header {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }

    /// Convert a string to a header
    ///
    /// String must be in the format `name: value`
    /// ## Example
    /// ```rust
    /// // Import Modules
    /// use afire::Header;
    ///
    /// let header1 = Header::new("Content-Type", "text/html");
    /// let header2 = Header::from_string("Content-Type: text/html").unwrap();
    ///
    /// assert!(header2 == header1);
    pub fn from_string(header: &str) -> Option<Header> {
        let splitted_header: Vec<&str> = header.split(':').collect();
        if splitted_header.len() != 2 {
            return None;
        }
        Some(Header {
            name: splitted_header[0].trim().to_string(),
            value: splitted_header[1].trim().to_string(),
        })
    }
}

impl fmt::Display for Header {
    /// Convert a header to a string
    ///
    /// Im format: `name: value`
    /// ## Example
    /// ```rust
    /// // Import Modules
    /// use afire::Header;
    ///
    /// let header1 = Header::new("Content-Type", "text/html");
    ///
    /// assert_eq!(header1.to_string(), "Content-Type: text/html");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Header) -> bool {
        self.name == other.name && self.value == other.value
    }
}

/// Stringify a Vec of headers
///
/// Each header is in the format `name: value`
///
/// Every header is separated by a newline (`\r\n`)
pub fn headers_to_string(headers: Vec<Header>) -> String {
    let headers_string: Vec<String> = headers.iter().map(|header| header.to_string()).collect();
    headers_string.join("\r\n")
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}
