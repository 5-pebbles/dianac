use std::{fs, path::Path, time::Instant};

use arbitrary_int::u6;
use colored::Colorize;

use crate::errors::Error;

use self::{analyzer::analyzer, assembler::assemble, diagnostic::DiagLevel, parser::Parser};

mod diagnostic;
mod span;

mod ir;
mod tokens;

mod analyzer;
mod assembler;
mod handlers;
mod lexer;
mod parser;

pub fn compile_impl(source: &Path, quiet: bool) -> Result<Option<Vec<u6>>, Error> {
    let absolute = fs::canonicalize(source)?.to_string_lossy().into_owned();
    println!("   {} `{absolute}`", "Compiling".green().bold(),);

    let start_time = Instant::now();

    let code = fs::read_to_string(&source)?;
    let parser = Parser::from(code.as_str());

    let (ir, mut diagnostics) = parser.parse();
    let (symbol_table, more_diagnostics) = analyzer(&ir);
    diagnostics.extend(more_diagnostics);

    let (instructions, more_diagnostics) = assemble(&ir, &symbol_table);
    let elapsed_time = start_time.elapsed();

    diagnostics.extend(more_diagnostics);

    let log_level = quiet
        .then_some(DiagLevel::Fatal)
        .unwrap_or(DiagLevel::Warning);

    let (mut errors, mut warnings) = (0, 0);
    diagnostics
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
            elapsed_time,
        );
        Some(instructions.into_iter().map(|i| i.raw_value()).collect())
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

pub fn compile_to_file(source: &Path, destination: &Path, quiet: bool) -> Result<(), Error> {
    if let Some(instructions) = compile_impl(source, quiet)? {
        fs::write(
            destination,
            instructions.iter().map(|i| i.value()).collect::<Vec<u8>>(),
        )?;
    }

    Ok(())
}
