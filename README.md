# Qalqulator

A calculator that uses (and outputs) rational numbers wherever possible.

The only operation that causes the output to revert to a floating point is exponentiation with fractional exponents.

## Installation

### Using Cargo

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Run the following command in your terminal:

   ```bash
   cargo install qalqulator
   ```

### Using pre-built binaries

Pre-built binaries are available on the [releases page](https://github.com/Gadiguibou/qalqulator/releases/).

## Example usage

```bash
$ qalqulator
>>> 6 * 7
42
>>> (12/11)^(4/2)
144/121
>>> let x = 141 % 100
41
>>> x + 0.5
83/2
>>> to_float(x) + 1/2
41.5
>>> x^(1/2)
6.4031242374328485
>>> exit
```

## Features

- Supports all basic arithmetic operations with operator precedence and parentheses:
  - Addition: `+`
  - Subtraction (and negation): `-`
  - Multiplication: `*`
  - Division: `/`
  - Remainder (not modulus): `%`
  - Exponentiation: `^`
- Numbers can be entered as integers (`42`), floating point numbers (`42.0`), or fractions (`42/1`) and will be automatically converted to rational numbers.
- Values can be assigned to variable names using the `let` keyword. For example, `let x = 42` will assign the value 42 to the variable `x`.
- To convert a value to a floating point number manually, use the `to_float` function. For example, `to_float(83/2)` will return `41.5`.
- To leave qalqulator, use `exit` or <kbd>Ctrl</kbd>+<kbd>D</kbd>.

## Additional features

This is a toy project that I built for my own needs over the course of a few hours. If you find it useful and would like to see additional features, please open an issue or a pull request.

## Name

Qalqulator is a calculator over $\mathbb{Q}$, the set of all rational numbers.
