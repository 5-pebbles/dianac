use std::collections::{hash_map::Entry, HashMap};

use arbitrary_int::u12;

use crate::{
    diagnostic::{DiagKind, DiagLevel, Diagnostic},
    ir::Ir,
    span::Span,
};

fn duplicate_label_error(span: Span) -> Diagnostic {
    Diagnostic {
        level: DiagLevel::Fatal,
        span,
        kind: DiagKind::DuplicateLabel,
    }
}

pub fn analyzer<'a>(ir: &Vec<Ir<'a>>) -> (HashMap<&'a str, u12>, Vec<Diagnostic>) {
    let mut symbol_table: HashMap<&'a str, u12> = HashMap::new();
    let mut diagnostics = Vec::new();

    ir.into_iter().fold(u12::new(0), |mut address, ir| {
        address += ir.len();
        if let Ir::Label(title, span) = ir {
            match symbol_table.entry(title) {
                Entry::Vacant(entry) => {
                    entry.insert(address);
                    ()
                }
                Entry::Occupied(_) => diagnostics.push(duplicate_label_error(span.clone())),
            }
        }
        address
    });

    (symbol_table, diagnostics)
}
