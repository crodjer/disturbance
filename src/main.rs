use disturbance::config::Config;
use disturbance::worker::workers;
use failure::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let config = Config::from_args();
    workers(config)
}
