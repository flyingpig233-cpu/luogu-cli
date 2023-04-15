mod cli;
mod menus;
mod luogu;
use cli::arg::Args;
use clap::Parser;
use cli::cli::LuoguCLI;


rust_i18n::i18n!("locales");

fn main() {
    let args = Args::parse();
    let cli = LuoguCLI::from(args);
    cli.run();
}
