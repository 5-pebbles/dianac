use std::{fs, path::Path};

use arbitrary_int::u6;
use colored::Colorize;

use crate::{
    compilation::{compile_to_binary, DiagLevel},
    errors::Error,
};

pub fn interpret(source: &Path, quiet: bool) -> Result<(), Error> {
    // TODO This should not panic on unwrap just return Ok(())
    emulation_repl(display_compilation(source, quiet)?.unwrap());

    Ok(())
}

pub fn emulate(source: &Path) -> Result<(), Error> {
    let machine_code = fs::read(source)?;
    // TODO This will panic if the first two bits are not zero; add a message for the user
    emulation_repl(machine_code.into_iter().map(|i| u6::new(i)).collect());
    Ok(())
}

pub fn compile(source: &Path, destination: &Path, quiet: bool) -> Result<(), Error> {
    if let Some(instructions) = display_compilation(source, quiet)? {
        fs::write(
            destination,
            instructions.iter().map(|i| i.value()).collect::<Vec<u8>>(),
        )?;
    }

    Ok(())
}

fn emulation_repl(machine_code: Vec<u6>) {
    todo!()
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
