use std::{char::CharTryFromError, sync::OnceLock};

use arbitrary_int::u6;
use bimap::BiMap;

pub fn character_table() -> &'static BiMap<u6, char> {
    macro_rules! insert {
        ($table:ident, {$($numeric:literal = $character:literal),* $(,)?}) => {
            $($table.insert(u6::new($numeric), $character));*
        };
    }
    static BIMAP: OnceLock<BiMap<u6, char>> = OnceLock::new();
    BIMAP.get_or_init(|| {
        let mut table = BiMap::new();
        // |    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   A   | B | C | D | E |  F  |
        // |:--:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:---:|:-:|:-----:|:-:|:-:|:-:|:-:|:---:|
        // | 0x | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   =   | - | + | * | / |  ^  |
        // | 1x | A | B | C | D | E | F | G | H |  I  | J |   K   | L | M | N | O |  P  |
        // | 2x | Q | R | S | T | U | V | W | X |  Y  | Z | SPACE | . | , | ' | " |  \` |
        // | 3x | # | ! | & | ? | ; | : | $ | % |  \| | > |   <   | [ | ] | ( | ) |  \\ |
        #[rustfmt::skip]
        insert!(table, {
            0x00 = '0', 0x01 = '1', 0x02 = '2', 0x03 = '3', 0x04 = '4', 0x05 = '5', 0x06 = '6', 0x07 = '7',
            0x08 = '8', 0x09 = '9', 0x0A = '=', 0x0B = '-', 0x0C = '+', 0x0D = '*', 0x0E = '/', 0x0F = '^',
            0x10 = 'A', 0x11 = 'B', 0x12 = 'C', 0x13 = 'D', 0x14 = 'E', 0x15 = 'F', 0x16 = 'G', 0x17 = 'H',
            0x18 = 'I', 0x19 = 'J', 0x1A = 'K', 0x1B = 'L', 0x1C = 'M', 0x1D = 'N', 0x1E = 'O', 0x1F = 'P',
            0x20 = 'Q', 0x21 = 'R', 0x22 = 'S', 0x23 = 'T', 0x24 = 'U', 0x25 = 'V', 0x26 = 'W', 0x27 = 'X',
            0x28 = 'Y', 0x29 = 'Z', 0x2A = ' ', 0x2B = '.', 0x2C = ',', 0x2D = '\'', 0x2E = '"', 0x2F = '`',
            0x30 = '#', 0x31 = '!', 0x32 = '&', 0x33 = '?', 0x34 = ';', 0x35 = ':', 0x36 = '$', 0x37 = '%',
            0x38 = '|', 0x39 = '>', 0x3A = '<', 0x3B = '[', 0x3C = ']', 0x3D = '(', 0x3E = ')', 0x3F = '\\'
        });
        table
    })
}

pub fn encode_character(character: &char) -> Option<&'static u6> {
    character_table().get_by_right(character)
}

pub fn decode_character(numeric: &u6) -> &'static char {
    character_table().get_by_left(numeric).unwrap()
}
