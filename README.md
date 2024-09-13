# Diana Compiled Language Reference Manual

A simple compiled language written for the [Diana-II](#diana-ii-specifications) 6-bit computer. This language was written to aid development by providing all basic instructions not natively supported by the architecture.

The following documentation is intended for programmers who are already familiar with other assembly like languages.

**Acknowledgments:** The following documentation is strongly inspired by the [Solaris x86
assembly language reference manual](https://docs.oracle.com/cd/E19253-01/817-5477/817-5477.pdf)


## Installation and Help

**From crates.io: (Recommended)**

```bash
cargo install dianac
```

**From source:**

```bash
git clone https://github.com/5-pebbles/dianac.git
cd dianac
cargo install --path .
```

**Basic help:**

```
~ ❯ dianac --help
An emulator, compiler, and interpreter for the Diana Compiled Language

Usage: dianac <COMMAND>

Commands:
  repl     Start the interactive emulation REPL
  compile  Compile a static binary (6-bit bytes are padded with zeros)
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**If there is anything that can be improved, please let me know:**

- `Issue:` [GitHub](https://github.com/5-pebbles/dianac/issues).
- `Email:` [5-pebble@protonmail.com](mailto:5-pebble@protonmail.com).


## Diana II Specifications

The Diana II is 6-bit minimal instruction set computer designed around using `NOR` as a universal logic gate.

- **byte size:** 6-bits.

- **endianness:** little-endian.

- **address size:** 12-bits (two 6-bit operands, first is higher order).

- **unique instructions:** 6.


### Instructions

| Binary |      Instruction     |  Description  |
|--------|----------------------|---------------|
|   00   |  `NOR [val] [val]`   |  Performs a negated OR on the first operand. |
|   01   |  `PC [val] [val]`    |  Sets the program counter to the address `[val, val]`. |
|   10   |  `LOAD [val] [val]`  |  Loads data from the address `[val, val]` into `C`. |
|   11   |  `STORE [val] [val]` |  Stores the value in `C` at the address `[val, val]`. |

**Layout:**

Each instruction is 6 bits in the format `[XX][YY][ZZ]`:

- **X:** 2-bit instruction identifier.
- **Y:** 2-bit first operand identifier.
- **Z:** 2-bit second operand identifier.

The first operand of NOR can't be immediate, so that allows another four instructions:

| Binary |   Instruction   | Description |
|--------|-----------------|-------------|
| 001100 | `NOP` | No operation; used for padding. |
| 001101 | `---` | Reserved for future use. |
| 001110 | `---` | Reserved for future use. |
| 001111 | `HLT` | Halts the CPU until the next interrupt. |


> [!Note]
> Instructions and operands are uppercase because my 6-bit character encoding does not support lowercase...


### Operands

| Binary | Name | Description |
|--------|------|-------------|
| **00** |   A  | General purpose register. |
| **01** |   B  | General purpose register. |
| **10** |   C  | General purpose register. |
| **11** |   -  | Read next instruction as a value. |


### Memory Layout

There are a total of 4096 unique address each containing 6 bits.

|     Address     |  Description  |
|-----------------|---------------|
| `0x000..=0xEFF` | General purpose RAM. |
| `0xEFF..=0xF3D` | Reserved for future use. |
| `0xF3E..=0xF3F` | Program Counter(PC) (ROM). |
| `0xF80..=0xFBF` | Left rotate lookup table (ROM). |
| `0xFC0..=0xFFF` | Right rotate lookup table (ROM). |


## Lexical Conventions

### Statements

A program consists of one or more files containing _statements_. A _statement_ consists of _tokens_ separated by whitespace and terminated by a newline character.

### Comments

A _comment_ can reside on its own line or be appended to a statement.  The comment consists of an octothorp (#) followed by the text of the comment and a terminating newline character.

### Labels

A _label_ can be placed before the beginning of a statement. During compilation the label is assigned the address of the following statement and can be used as a keyword operand.
A label consists of the `LAB` keyword followed by an _identifier_ labels are global in scope and appear in the files symbol table.

### Tokens

There are 6 classes of tokens:

- Identifiers
- Keywords
- Registers
- Numerical constants
- Character constants
- Operators

#### Identifiers

An identifier is an arbitrarily-long sequence of letters, underscores, and digits. The first character must be letter or underscore. Uppercase and lowercase characters are equivalent.

#### Keywords

Keywords such as instruction mnemonics and directives are reserved and cannot be used as identifiers. For a list of keywords see the [Keyword Tables](#keyword-tables).

#### Registers

The Diana-II architecture provides three registers **\[A, B, C\]** these are reserved and can not be used as identifiers. Uppercase and lowercase characters are equivalent.

#### Numerical Constants

Numbers in the Diana-II architecture are unsigned 6-bit integers. These can be expressed in several bases:

- **Decimal.** Decimal integers consist of one or more decimal digits (0–9).
- **Binary.** Binary integers begin with “0b” or “0B” followed by zero or more binary digits (0, 1).
- **Hexadecimal.** Hexadecimal integers begin with “0x” or “0X” followed by one or more hexadecimal digits (0–9, A–F). Hexadecimal digits can be either uppercase or lowercase.

#### Character Constants

A _character_ constant consists of a supported character enclosed in single quotes ('). A character will be converted to its numeric representation based on the table of supported characters bellow:

|    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   A   | B | C | D | E |  F  |
|:--:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:---:|:-:|:-----:|:-:|:-:|:-:|:-:|:---:|
| 0x | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   =   | - | + | * | / |  ^  |
| 1x | A | B | C | D | E | F | G | H |  I  | J |   K   | L | M | N | O |  P  |
| 2x | Q | R | S | T | U | V | W | X |  Y  | Z | SPACE | . | , | ' | " |  \` |
| 3x | # | ! | & | ? | ; | : | $ | % |  \| | > |   <   | [ | ] | ( | ) |  \\ |

If a lowercase character is used, it will be converted to its uppercase representation.

#### Operators

The compiler supports the following operators for use in expressions. Operators have no assigned precedence. Expressions can be grouped in parentheses () to establish precedence.

|     |     |
|-----|-----|
|  !  | Logical NOT |
|  &  | Logical AND |
|  \| | Logical OR |
|  +  | Addition |
|  -  | Subtraction |
|  *  | Multiplication |
|  /  | Division |
|  >> | Rotate right |
|  << | Rotate left |

All operators except Logical NOT require two values and parentheses ():

- `(5 + 9 + 3)` = **17**
- `!0b111110` = **0b000001**
- `(2 + (2 * 5))` = **12**
- `(2 + 2 * 5)` = **20**


## Keywords, Operands, and Addressing

Keywords represent an instruction, set of instructions, or a directive. Operands are entities operated upon by the keyword. Addresses are the locations in memory of specified data.

### Operands

A keyword can have zero to three operands separated by whitespace characters. For instructions with a source and destination this language uses Intel's notation destination(lefthand) then source(righthand).

There are 4 types of operands:

- **Immediate.** A 6-bit constant expression that evaluate to an inline value.
- **Register.** One of the three 6-bit general-purpose registers provided by the Diana-II architecture.
- **Either.** An immediate or a register operand.
- **Address.** A single 12-bit identifier or two a pair of whitespace separated 6-bit either operands.
- **Conditional.** A pair of square brackets \[ \] containing a pair of 6-bit operands separated by whitespace and one of the following comparison operators:
    |      |      |
    |------|------|
    |  ==  | Equal |
    |  !=  | Not equal |
    |  >   | Greater |
    |  >=  | Greater or equal |
    |  <   | Less |
    |  <=  | Less or equal |

### Addressing

The Diana-II architecture uses 12-bit addressing. Labels can be split into two 6-bit immediate values by appending a colon followed by a 1 or 0. If a keyword requires an address it can be provided as two 6-bit values or a single 12-bit identifier: 

- `LOD MAIN` = `LOD MAIN:0 MAIN:1`.


## Side Effects

Any side effects will be listed in the notes of a keyword read each carefully. If a keyword clobbers an unrelated register, it will select the first available in reverse alphabetical order, e.g.

- `XNOR C 0x27` will clobber **B**
- `XNOR A 0x27` will clobber **C**


## Keyword Tables

Operands will be displayed in square brackets \[ \] using the following shorthand:

- `[reg]` = **register**
- `[imm]` = **immediate**
- `[eth]` = **either**
- `[add]` = **address**
- `[con]` = **conditional**

### Bitwise Logic Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `NOT [reg]` | bitwise logical NOT | - |
| `AND [reg] [eth]` | bitwise logical AND | The second register is flipped; its value can be restored with a `NOT` operation. If an immediate value is used, it is flipped at compile time. |
| `NAND [reg] [eth]` | bitwise logical NAND | The second register is flipped; its value can be restored with a `NOT` operation. If an immediate value is used, it is flipped at compile time. |
| `OR [reg] [eth]` | bitwise logical OR | - |
| `NOR [reg] [eth]` | bitwise logical NOR | - |
| `XOR [reg] [eth]` | bitwise logical XOR | An extra register will be clobbered; this is true even if an immediate value is used. |
| `NXOR [reg] [eth]` | bitwise logical NXOR | An extra register will be clobbered; this is true even if an immediate value is used. |

### Shift and Rotate Keywords

These keywords simply load the corresponding address from the right and left rotate [lookup tables](#memory-layout).

| Keyword | Description | Notes |
|---------|-------------|-------|
| `ROL [eth]` | rotate left storing the value in **C**  | - |
| `ROR [eth]` | rotate right storing the value in **C** | - |
| `SHL [eth]` | shift left storing the value in **C**   | - |
| `SHR [eth]` | shift right storing the value in **C**  | - |

### Arithmetic Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `ADD [reg] [eth]` | add | All registers will be clobbered; this is true even if an immediate value is used. |
| `SUB [reg] [eth]` | subtract | All registers will be clobbered; this is true even if an immediate value is used. |

### Memory Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `SET [imm]` | compiles to raw value `[imm]` | - |
| `MOV [reg] [eth]` | copy from second operand to first | - |
| `LOD [add]` | load data from `[add]` into **C** | - |
| `STO [add]` | stores data in **C** at `[add]` | - |

### Jump Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `PC [add]`  | set program counter to `[add]` | - |
| `LAB [idn]` | define a label pointing to the next statement | - |
| `LIH [con] [add]` | conditional jump if true | All registers will be clobbered, and LIH stands for logic is hard. |

### Miscellaneous Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `NOP` | No operation; used for padding | - |
| `HLT` | halts the CPU until the next interrupt | - |
