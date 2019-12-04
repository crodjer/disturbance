use structopt::StructOpt;

/// Monitor disturbances in a web service's behaviour.
#[derive(Clone, StructOpt, Debug)]
pub struct Config {
    /// The web service's URL to monitor
    pub url: String,
    /// Response should match
    #[structopt(short, long)]
    pub matches: Option<String>,
    /// Response should not match
    #[structopt(short, long)]
    pub excludes: Option<String>,
    /// Wait time (in ms) between requests per worker.
    #[structopt(short = "w", long = "wait", default_value = "100")]
    pub wait: u64,
    /// Request timeout in seconds
    #[structopt(short = "t", long = "timeout", default_value = "5")]
    pub timeout: usize,
    /// Parallelism
    #[structopt(short = "p", long = "parallelism", default_value = "2")]
    pub parallelism: usize,
}
