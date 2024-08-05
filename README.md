# Diana Compiled Language Reference Manual

A simple compiled language written for the [Diana-II](https://github.com/5-pebbles/diana-ii) 6-bit minimal instruction set computer. This language was written to aid development by providing all basic instructions not natively supported by the architecture.

The following documentation is intended for programmers who are already familiar with other assembly like languages. You are also expected to have read the CPUs documentation which can be found [here](https://github.com/5-pebbles/diana-ii).

**Acknowledgments:** The following documentation is strongly inspired by the [Solaris x86
assembly language reference manual](https://docs.oracle.com/cd/E19253-01/817-5477/817-5477.pdf)


## Installation

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
|  +  | Addition |
|  -  | Subtraction |
|  *  | Multiplication |
|  /  | Division |
|  &  | Logical AND |
|  \| | Logical OR |
|  >> | Shift right |
|  << | Shift left |
|  %  | Modulo |


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

### Logical Keywords

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

These keywords simply load the corresponding address from the right and left rotate [lookup tables](https://github.com/5-pebbles/diana-ii#memory-layout).

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
| `ADS [reg] [eth]` | saturated add | All registers will be clobbered; this is true even if an immediate value is used. |
| `SUB [reg] [eth]` | subtract | All registers will be clobbered; this is true even if an immediate value is used. |
| `SBS [reg] [eth]` | saturated subtract | All registers will be clobbered; this is true even if an immediate value is used. |

### Memory Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `SET [imm]` | compiles to raw value `[imm]` | - |
| `MOV [reg] [eth]` | copy from second operand to first | - |
| `LOD [add]` | load data from `[add]` into **C** | - |
| `STO [add]` | stores data in **C** at `[add]` | - |

### Jumps and Logic Keywords

| Keyword | Description | Notes |
|---------|-------------|-------|
| `PC [add]`  | set program counter to `[add]` | - |
| `LAB [idn]` | define a label pointing to the next statement | - |
