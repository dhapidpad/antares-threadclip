use tc_core::{Result, TcError};

pub struct MockThreadsClient {
    pub simulate_rate_limit: bool,
    pub simulate_token_expired: bool,
}

impl MockThreadsClient {
    pub fn new() -> Self {
        Self {
            simulate_rate_limit: false,
            simulate_token_expired: false,
        }
    }

    pub fn fetch_user_posts(&self, _user_id: &str) -> Result<Vec<String>> {
        if self.simulate_token_expired {
            return Err(TcError::TokenExpired(
                "Mock token expired (HTTP 401)".to_string(),
            ));
        }
        if self.simulate_rate_limit {
            return Err(TcError::RateLimited(60));
        }
        Ok(vec!["post_1".to_string(), "post_2".to_string()])
    }
}
