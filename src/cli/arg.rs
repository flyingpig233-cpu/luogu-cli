use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Luogu CLI",
          author = "Chengmin Xu <xuchengmin39@gmail.com>",
          version = "1.0",
          about = "A powerful tool for Luogu OJ", long_about = None)]
pub struct Args {
    /// Set a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>
}
