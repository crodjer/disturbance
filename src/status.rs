use crate::config::Config;
use reqwest;

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
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
        Status::Success
    }
}

impl From<reqwest::Error> for Status {
    fn from(err: reqwest::Error) -> Self {
        Status::ErrorResponse(err.to_string())
    }
}

impl Status {
    fn check_result(config: &Config) -> Result<Status, Status> {
        let mut resp = reqwest::get(&config.url)?;
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
        };

        assert_eq!(
            categorise_response(200, "Foo Bar", &config),
            Status::Success
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
