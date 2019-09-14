use disturbance::config::Config;
use disturbance::status::Status;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("Status: {:?}", Status::check(&config));
}
