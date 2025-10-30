use clap::{Parser};

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value = "metrics.db")]
    pub db: String
}
