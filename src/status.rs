use crate::config::Config;
use reqwest;
use std::fmt;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Success(u16),
    DoesNotMatch,
    DoesNotExclude,
    ErrorStatus(u16),
    ErrorResponse(String),
}

fn categorise_response(status: u16, text: &str, config: &Config) -> Status {
    if status >= 400 {
        Status::ErrorStatus(status)
    } else if config
        .matches
        .as_ref()
        .map_or(false, |pattern| !text.contains(pattern))
    {
        Status::DoesNotMatch
    } else if config
        .excludes
        .as_ref()
        .map_or(false, |pattern| text.contains(pattern))
    {
        Status::DoesNotExclude
    } else {
        Status::Success(status)
    }
}

impl From<reqwest::Error> for Status {
    fn from(err: reqwest::Error) -> Self {
        Status::ErrorResponse(err.to_string())
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Status {
    fn check_result(config: &Config) -> Result<Status, Status> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;
        let mut resp = client.get(&config.url).send()?;
        let text = resp.text()?;

        Ok(categorise_response(resp.status().as_u16(), &text, config))
    }

    pub fn check(config: &Config) -> Status {
        Self::check_result(config).unwrap_or_else(|status| status)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_categorize_response_with_matches_and_excludes() {
        let config = Config {
            url: "https://example.com".to_string(),
            matches: Some("Foo".to_string()),
            excludes: Some("Baz".to_string()),
            parallelism: 1,
            timeout: 5,
        };

        assert_eq!(
            categorise_response(200, "Foo Bar", &config),
            Status::Success(200)
        );

        assert_eq!(
            categorise_response(200, "Bar", &config),
            Status::DoesNotMatch
        );

        assert_eq!(
            categorise_response(200, "Foo Bar Baz", &config),
            Status::DoesNotExclude
        );

        assert_eq!(
            categorise_response(500, "", &config),
            Status::ErrorStatus(500)
        )
    }
}
