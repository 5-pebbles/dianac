use arbitrary_int::u6;
use clap::{Parser as ArgParser, Subcommand};
use colored::Colorize;
use std::{
    fs,
    io::{stdin, stdout, BufRead, Write},
    path::{Path, PathBuf},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

#[cfg(test)]
mod tests;

mod compilation;
mod emulation;

mod character_encoding;
mod errors;
mod instruction;

use errors::Error;

use crate::{
    compilation::{compile_to_binary, DiagLevel},
    emulation::InteractiveState,
};

/// An emulator, compiler, and interpreter for the Diana Compiled Language
#[derive(ArgParser)]
#[command(version, about, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the interactive emulation REPL
    Repl,
    /// Compile a static binary (6-bit bytes are padded with zeros)
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
        Command::Repl => emulation_repl()?,
        Command::Compile {
            source,
            destination,
            quiet,
        } => {
            if let Some(instructions) = display_compilation(&source, quiet)? {
                fs::write(
                    destination.unwrap_or_else(|| source.with_extension("")),
                    instructions.iter().map(|i| i.value()).collect::<Vec<u8>>(),
                )?;
            }
        }
    }

    Ok(())
}

macro_rules! bold {
    ($e:expr) => {
        concat!("\x1B[1m", $e, "\x1B[0m")
    };
}

macro_rules! about {
    (repl) => {
        concat!(
            "Diana-II Emulation Repl (v",
            env!("CARGO_PKG_VERSION"),
            "):\n",
            about!(commands),
        )
    };
    (commands) => {
        concat!(
            bold!("- run | r [speed]:"),
            " run at speed (hz) until next halt\n",
            bold!("- step | s:"),
            " step one instruction\n",
            bold!("- interpret | i <dcl_source> [offset]:"),
            " compile and load at the given offset\n",
            bold!("- help | h:"),
            " print help\n",
            bold!("- quit | q:"),
            " close repl\n",
        )
    };
}

pub fn emulation_repl() -> Result<(), Error> {
    // repl (read–eval–print loop)
    println!(about!(repl));
    let mut state = InteractiveState::new();

    loop {
        print!("> ");
        stdout().flush()?;

        let mut line = String::new();
        stdin().lock().read_line(&mut line)?;

        let args: Vec<&str> = line.split_whitespace().collect();

        let command = match args.get(0) {
            Some(value) => value.to_lowercase(),
            None => continue,
        };

        // add your changes to about!(commands) ╾━╤デ╦︻(▀̿Ĺ̯▀̿ ̿) or else!
        match command.as_str() {
            "run" | "r" => {
                let sleep_time = if let Some(speed) = args.get(1) {
                    1.0 / f64::from_str(*speed).unwrap()
                } else {
                    0.0
                };

                state.consume_instruction();
                while !state.is_halt() {
                    sleep(Duration::from_secs_f64(sleep_time));
                    state.consume_instruction();
                }

                let (p0, p1) = state.program_counter.as_tuple();
                println!("Reached Halt {:0>6b}-{:0>6b}", p0, p1);
            }
            "step" | "s" => {
                state.consume_instruction();
            }
            "interpret" | "i" => {
                let dcl_file = args.get(1).unwrap();
                let offset = if let Some(offset) = args.get(2) {
                    usize::from_str_radix(offset, 10).unwrap()
                } else {
                    0
                };

                let machine_code = display_compilation(Path::new(dcl_file), false)?.unwrap();

                state.memory.store_array(offset, &machine_code)
            }
            "help" | "h" => println!(about!(commands)),
            "quit" | "q" => break,
            "" => continue,
            unknown => println!("Unknown Command: \"{unknown}\""),
        };
    }

    Ok(())
}

fn display_compilation(source: &Path, quiet: bool) -> Result<Option<Vec<u6>>, std::io::Error> {
    let absolute = fs::canonicalize(source)?.to_string_lossy().into_owned();
    println!("   {} `{absolute}`", "Compiling".green().bold(),);

    let code = fs::read_to_string(&source)?;
    // This makes parsing case independent; the original code is saved for diagnostics
    let code_uppercase = code.to_uppercase();

    let compile_info = compile_to_binary(&code_uppercase);

    let log_level = quiet
        .then_some(DiagLevel::Fatal)
        .unwrap_or(DiagLevel::Warning);
    let (mut errors, mut warnings) = (0, 0);
    compile_info
        .diagnostics
        .into_iter()
        .filter(|diag| diag.level <= log_level)
        .for_each(|diag| {
            diag.emit(&code, source);
            match diag.level {
                DiagLevel::Fatal => errors += 1,
                DiagLevel::Warning => warnings += 1,
            }
        });

    let warning_plural = if warnings > 1 { "warnings" } else { "warning" };
    if !quiet && warnings > 0 {
        println!(
            "{} generated {warnings} {warning_plural}",
            format!("{}:", "warning".yellow()).bold(),
        )
    }

    Ok(if errors == 0 {
        println!(
            "    {} `{absolute}` in {:?}",
            "Finished".green().bold(),
            compile_info.duration,
        );
        Some(compile_info.binary)
    } else {
        let error_plural = if errors > 1 { "errors" } else { "error" };
        println!(
            "{} could not compile due to {errors} previous {error_plural}{}",
            format!("{}:", "error".red()).bold(),
            if warnings > 0 {
                format!("; {warnings} {warning_plural} emitted")
            } else {
                "".to_string()
            }
        );
        None
    })
}
