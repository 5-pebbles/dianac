# Diana Compiled Language .DCL

A simple compiled language and emulator written for the [Diana-II](https://github.com/5-pebbles/diana-ii) custom CPU.

**Example:**
```
# FIBONACCI SEQUENCE
LABEL MAIN
LOAD ITERATIONS
LIH [C == 0] END
SUB C 1
STORE ITERATIONS

LOAD LAST
MOVE A C
LOAD THIS
STORE LAST
SATADD C A
STORE THIS

PC MAIN

LABEL END
PC END

LABEL ITERATIONS
SET 5 # SET YOUR OWN VALUE BUT 9 IS THE MAX STORABLE IN 6 BITS

LABEL THIS
SET 1
LABEL LAST
SET 0
```

If you want to play around with this language, you should really read the CPUs [documentation](https://github.com/5-pebbles/diana-ii) first ;)


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


## Language

This language is meant to aid development by providing all basic instructions not natively supported by the architecture. However, it does not contain any abstractions that could hurt performance.


**Character encoding:**

There are a few characters that can be used as immediate values. Below is a table with each and its hexadecimal representation:

|    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   A   | B | C | D | E |  F  |
|:--:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:---:|:-:|:-----:|:-:|:-:|:-:|:-:|:---:|
| 0x | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  8  | 9 |   =   | - | + | * | / |  ^  |
| 1x | A | B | C | D | E | F | G | H |  I  | J |   K   | L | M | N | O |  P  |
| 2x | Q | R | S | T | U | V | W | X |  Y  | Z | SPACE | . | , | ' | " |  \` |
| 3x | # | ! | & | ? | ; | : | $ | % |  \| | > |   <   | [ | ] | ( | ) |  \\ |

If a lowercase character is used, it will be converted to its uppercase representation.


**Immediate expressions:**

Expressions within a parentheses block are evaluated at compile time. The not (**!**) operator is the one exception to this not requiring a block.

- `(0b110000 | 0b000011)` = `0b110011`
- `!0b110000` = `0b001111`


**Address expansion:**

An address tuple (\<register | immediate> \<register | immediate>) can be replaced with a raw address, e.g.

- `PC MAIN:0 MAIN:1` = `PC MAIN`

> [!Important]
> This notation does not yet support immediate expressions.

**Side effects:**

If an instruction clobbers an unrelated register, it will select the first available in reverse alphabetical order, e.g.

- `XNOR C 0x27` will clobber **B**
- `XNOR A 0x27` will clobber **C**


**List of instructions and side effects:**

- [NOR](#nor-register-register--immediate)
- [PC](#pc-register--immediate-register--immediate)
- [LOAD](#load-register--immediate-register--immediate)
- [STORE](#store-register--immediate-register--immediate)


## Instructions

### NOR \<register\> \<register | immediate\>

Performs a logical `NOR` on the provided values, storing the result in the first register.

| p | q | NOR |
|---|---|-----|
| 1 | 1 |  0  |
| 1 | 0 |  0  |
| 0 | 1 |  0  |
| 0 | 0 |  1  |

**Usage:**
```
NOR A B
NOR B 0b110101
```

**Example:**
```
00-00-10
00-01-11
11-01-01
```

&nbsp;
### PC \<register | immediate\> \<register | immediate\>

Jumps the program counter to the provided 12-bit address tuple.

**Usage:**
```
PC MAIN
LABEL MAIN
PC A 0x1F
```

**Example:**
```
01-11-11
00-00-00
00-00-11
01-00-11
00-11-11
```

&nbsp;
### LOAD \<register | immediate\> \<register | immediate\>

Loads data from the provided 12-bit address tuple into register C.

**Usage:**
```
LOAD VALUE # C = 10-00-11
LABEL VALUE
LOAD A 0 # C = 10-11-11
```

**Example:**
```
10-11-11
00-00-00
00-00-11
10-00-11
00-00-00
```

&nbsp;
### STORE \<register | immediate\> \<register | immediate\>

Stores the value in register C at the 12-bit address tuple.

**Usage:**
```
STORE 0 3
LABEL MAIN
STORE C 0b011010 # This value is replaced, but its immidate value creates a loop
```

**Example:**
```
11-11-11
00-00-00
00-00-11
11-10-11
01-10-10
```
