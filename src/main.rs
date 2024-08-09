use clap::{Parser as ArgParser, Subcommand};
use std::path::PathBuf;

mod compilation;
mod emulation;

mod character_encoding;
mod errors;
mod instruction;

use compilation::{compile_impl, compile_to_file};
use emulation::{emulate_file, emulate_impl};
use errors::Error;

/// An emulator, compiler, and interpreter for the Diana Compiled Language
#[derive(ArgParser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Interprate a program directly from source
    Interpret {
        /// An input file handle (can be /dev/stdin)
        source: PathBuf,
        /// Suppress all non-fatal diagnostics
        #[arg(short, long)]
        quiet: bool,
    },
    /// Emulate the execution from a binary
    Emulate {
        /// An input file handle (can be /dev/stdin)
        source: PathBuf,
    },
    /// Compile a binary without execution
    Compile {
        /// An input file handle (can be /dev/stdin)
        source: PathBuf,
        /// File path to compiled binary
        destination: Option<PathBuf>,
        /// Suppress all non-fatal diagnostics
        #[arg(short, long)]
        quiet: bool,
    },
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Command::Interpret { source, quiet } => {
            emulate_impl(compile_impl(&source, quiet)?.unwrap());
        }
        Command::Emulate { source } => {
            emulate_file(&source)?;
        }
        Command::Compile {
            source,
            destination,
            quiet,
        } => {
            compile_to_file(
                &source,
                &destination.unwrap_or_else(|| source.with_extension("")),
                quiet,
            )?;
        }
    }

    Ok(())
}
