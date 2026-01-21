use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser)]
pub struct Config {
    #[arg(short, long, default_value="0.0.0.0:8080")]
    pub server_address: String,

    #[arg(short='d', long, default_value=".")]
    pub shared_dir: String,

    #[arg(short, long, default_value="/")]
    pub url_path: String,

    #[arg(short, long, default_value="index.html")]
    pub index_file: String
}

impl Config {
    pub fn tell_about_self(&self) {
        println!("{}", "[SRFS]".green().bold());
        println!("{} {}", "Listening on:".purple().bold(), &self.server_address.blue());
        println!("{} {}", "Serving from:".purple().bold(), &self.shared_dir.blue());
        println!("{} {}", "At URL:".purple().bold(), &self.url_path.blue());
        println!("{} {}", "With Index:".purple().bold(), &self.index_file.blue())
    }
}
