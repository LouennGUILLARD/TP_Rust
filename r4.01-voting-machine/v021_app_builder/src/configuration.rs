use clap::Parser;

#[derive(Parser)]
pub struct Configuration {
    #[clap(short = 'c', long = "candidates", required = true)]
    pub candidates: Vec<String>,
}

