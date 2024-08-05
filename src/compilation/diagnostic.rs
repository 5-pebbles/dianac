use std::{num::IntErrorKind, path::Path};

use colored::{Color, Colorize};
use strum::Display as EnumDisplay;

use crate::compilation::{span::Span, tokens::TokenKind};

pub fn emit_diagnostics(
    diagnostics: &Vec<Diagnostic>,
    raw: &str,
    path: &Path,
    log_level: DiagLevel,
) {
    diagnostics
        .into_iter()
        .filter(|diag| diag.level <= log_level)
        .for_each(|diag| diag.emit(raw, path))
}

#[derive(Debug, PartialEq, Clone)]
pub struct Diagnostic {
    pub level: DiagLevel,
    pub span: Span,
    pub kind: DiagKind,
}

impl Diagnostic {
    #[rustfmt::skip]
    pub fn emit(&self, raw: &str, path: &Path) {
        assert!(self.span.end <= raw.len());

        let (line_number, start_index) = self.get_line_info(raw);
        let line_number = line_number.to_string();
        let header = format!("{}: {}", self.level.to_string().color(self.level.color()), self.kind.to_string()).bold();
        let file_path = format!(" {}{} {}", " ".repeat(line_number.len()), "-->".bold().blue(), path.display());
        let prefix = format!(" {} |", " ".repeat(line_number.len())).blue().bold();
        let details = format!("{}{}", format!(" {} | ", line_number).blue().bold(), &raw[self.span.as_range()]);
        let highlight = format!("{prefix} {}{}", " ".repeat(self.span.start - start_index), "^".repeat(self.span.end - self.span.start).red());
        let help = format!("{}: {}", "help".cyan().bold(), self.kind.help());

        println!("{header}\n{file_path}\n{prefix}\n{details}\n{highlight}\n{prefix}\n{help}\n");
    }

    fn get_line_info(&self, raw: &str) -> (u16, usize) {
        raw.char_indices()
            .take_while(|(index, _)| index < &self.span.start)
            .fold((1, 0), |(line, _), (index, ch)| {
                (line + u16::from(ch == '\n'), index + 1)
            })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, EnumDisplay)]
pub enum DiagLevel {
    Fatal = 0,
    Warning = 1,
}

impl DiagLevel {
    pub fn color(&self) -> Color {
        match self {
            DiagLevel::Fatal => Color::Red,
            DiagLevel::Warning => Color::Yellow,
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum DiagKind {
    DuplicateLabel,
    UndefinedLabel,
    UnexpectedToken {
        found: TokenKind,
        expected: &'static str,
    },
    ParseImmediate(IntErrorKind),
}

impl DiagKind {
    pub fn help(&self) -> String {
        match self {
            DiagKind::UnexpectedToken { found, expected } => {
                format!("Expected `{expected}` found `{found}`")
            }
            DiagKind::ParseImmediate(error) => {
                format!("Error parsing value {:#?}", error)
            }
            _ => todo!(),
        }
    }
}
