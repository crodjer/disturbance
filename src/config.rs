use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
pub struct Config {
    /// The URL to hit
    pub url: String,
    /// Response should match
    #[structopt(short, long)]
    pub matches: Option<String>,
    /// Response should not match
    #[structopt(short, long)]
    pub excludes: Option<String>,
}
