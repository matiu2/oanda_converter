use parse_display::Display;

/// Whether to use the dev or live hosts
/// See: <https://developer.oanda.com/rest-live-v20/development-guide/>
#[derive(Debug, Clone, Copy, Display)]
pub enum Host {
    Dev,
    Live,
}

impl Host {
    /// Returns the API endpoint for the REST API
    /// See: <https://developer.oanda.com/rest-live-v20/development-guide/>
    pub fn rest(&self) -> &'static str {
        match self {
            Host::Dev => "api-fxpractice.oanda.com",
            Host::Live => "api-fxtrade.oanda.com",
        }
    }
    /// Returns the streaming API
    /// See: <https://developer.oanda.com/rest-live-v20/development-guide/>
    pub fn streaming(&self) -> &'static str {
        match self {
            Host::Dev => "stream-fxpractice.oanda.com",
            Host::Live => "stream-fxtrade.oanda.com",
        }
    }
    /// Generates a URL using the current host, `https` and your `path`
    pub fn rest_url(&self, path: impl std::fmt::Display) -> String {
        format!("https://{}{path}", self.rest())
    }
}
