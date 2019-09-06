use disturbance::config::Config;
use disturbance::status::{categorise_response, Status};
use reqwest;
use structopt::StructOpt;

fn verify(config: &Config) -> Result<Status, Status> {
    reqwest::get(&config.url)
        .and_then(|mut resp| resp.text())
        .map_err(|err| Status::ErrorResponse(err.to_string()))
        .and_then(|text| categorise_response(text, config))
}

fn main() {
    let config = Config::from_args();
    println!("Status: {:?}", verify(&config));
}
