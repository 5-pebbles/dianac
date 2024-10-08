use std::{
    collections::HashMap,
    sync::Arc,
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
pub mod generator;
pub mod lexer;
pub mod parser;

pub use diagnostic::{DiagLevel, Diagnostic};

#[allow(dead_code)]
pub struct CompileInfo {
    pub duration: Duration,
    pub symbol_table: HashMap<Arc<str>, u12>,
    pub binary: Vec<u6>,
    pub instructions: Vec<Instruction>,
    pub ir: Vec<Ir>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn compile_to_binary(source: &str, offset: u12) -> CompileInfo {
    let start_time = Instant::now();

    let tokens = Cursor::new(&source).tokenize().collect();

    let parser_result = Parser::new(&source, offset).parse();
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
