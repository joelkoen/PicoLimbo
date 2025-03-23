use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub address: String,
    #[arg(short, long)]
    pub json: bool,
    #[arg(short, long)]
    pub version: Option<String>,
}
