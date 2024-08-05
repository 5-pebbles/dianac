use std::{fs, path::Path};

use crate::errors::Error;

use self::{
    analyzer::analyzer,
    assembler::assemble,
    diagnostic::{emit_diagnostics, DiagLevel},
    parser::Parser,
};

mod diagnostic;
mod span;

mod ir;
mod tokens;

mod analyzer;
mod assembler;
mod lexer;
mod parser;

mod instruction;

pub fn compile_file(source: &Path, destination: &Path, quiet: bool) -> Result<(), Error> {
    let code = fs::read_to_string(&source)?;
    let parser = Parser::from(code.as_str());

    let (ir, mut diagnostics) = parser.parse();
    let (symbol_table, more_diagnostics) = analyzer(&ir);
    diagnostics.extend(more_diagnostics);

    let (instructions, more_diagnostics) = assemble(&ir, &symbol_table);
    diagnostics.extend(more_diagnostics);

    emit_diagnostics(
        &diagnostics,
        code.as_str(),
        &source,
        quiet
            .then_some(DiagLevel::Fatal)
            .unwrap_or(DiagLevel::Warning),
    );

    if !diagnostics.iter().any(|d| d.level == DiagLevel::Fatal) {
        fs::write(
            destination,
            instructions
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )?;
    }
    // TODO make a failure message

    Ok(())
}
