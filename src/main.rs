// In this codebase, comments have two purposes:
// 1. describe why something is done
// 2. boost developer morale; people do better work when they are having fun (jokes are encouraged)

#![allow(dead_code)]
mod compilation;

mod errors;
mod instruction;

use clap::Parser as ArgParser;
use compilation::compile_file;
use errors::Error;
use std::path::PathBuf;

#[derive(ArgParser)]
#[command(version, about)]
struct Args {
    source: PathBuf,
    #[arg(short, long)]
    destination: Option<PathBuf>,
    #[arg(short, long)]
    quiet: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    compile_file(
        &args.source,
        &args
            .destination
            .unwrap_or_else(|| args.source.with_extension("bin")),
        args.quiet,
    )?;

    Ok(())
}
