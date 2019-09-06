use crate::config::Config;

#[derive(Debug)]
pub enum Status {
    Success,
    DoesNotMatch,
    DoesNotExclude,
    ErrorResponse(String),
}

pub fn categorise_response(resp_text: String, config: &Config) -> Result<Status, Status> {
    if config
        .matches
        .as_ref()
        .map_or(false, |pattern| !resp_text.contains(pattern))
    {
        Err(Status::DoesNotMatch)
    } else if config
        .excludes
        .as_ref()
        .map_or(false, |pattern| resp_text.contains(pattern))
    {
        Err(Status::DoesNotExclude)
    } else {
        Ok(Status::Success)
    }
}
