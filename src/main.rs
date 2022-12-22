use clap::Parser;
use pngme::{Result, args, commands};

fn main() -> crate::Result<()> {
    let args = args::Args::parse();
    commands::run(args)
}   
