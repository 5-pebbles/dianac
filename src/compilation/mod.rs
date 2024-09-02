use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use arbitrary_int::{u12, u6};

use super::instruction::Instruction;

use self::{assembler::assemble, ir::Ir, lexer::Cursor, parser::Parser, tokens::Token};

mod diagnostic;
mod span;

mod ir;
mod tokens;

mod assembler;
mod generator;
mod lexer;
mod parser;

pub use diagnostic::{DiagKind, DiagLevel, Diagnostic};

pub struct CompileInfo<'a> {
    pub duration: Duration,
    pub symbol_table: HashMap<&'a str, u12>,
    pub binary: Vec<u6>,
    pub instructions: Vec<Instruction>,
    pub ir: Vec<Ir<'a>>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn compile_to_binary(source: &str) -> CompileInfo {
    let start_time = Instant::now();

    let tokens = Cursor::new(&source).tokenize().collect();

    let parser_result = Parser::new(&source).parse();
    let (ir, symbol_table, mut diagnostics) = (
        parser_result.ir,
        parser_result.symbol_table,
        parser_result.diagnostics,
    );

    let (instructions, more_diagnostics) = assemble(&ir, &symbol_table);
    diagnostics.extend(more_diagnostics);

    let binary = instructions.iter().map(|i| i.raw_value()).collect();

    let duration = start_time.elapsed();

    CompileInfo {
        duration,
        symbol_table,
        binary,
        instructions,
        ir,
        tokens,
        diagnostics,
    }
}
