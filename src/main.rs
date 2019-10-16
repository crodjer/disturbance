use disturbance::config::Config;
use disturbance::worker::workers;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    workers(config);
}
