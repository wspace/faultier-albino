Albino
================

This is a compiler front-end for the esoteric languages (Whitespace, Brainfuck, Ook!, [DT](https://faultier.blog.jp/archives/1139763.html)) which is build as part of the [Whitebase](https://github.com/wspace/faultier-whitebase) compiler infrastructure project.

This is a fork of faultier's [Albino](https://github.com/faultier/albino)
project, which updates it from [Rust](https://www.rust-lang.org/) v0.12.0-pre to
modern Rust.

## Features

- The interpreter for Whitespace, Brainfuck, Ook! and DT.
- The compiler and decompiler for these.
- The assembly language for Whitespace.

## Quick Start

```shell
cargo build
target/albino run examples/hello.ws
```

## Usage

### Run script

You can run the script directly. Type `albino run` command.

```shell
albino run hello.ws
```

`albino` can detect file type using extention, but you could specify any other.

```shell
albino run -s dt hello.ws
```

### Compile and execute

`albino` compiles the source code to Whitebase bytecode, and it can be executed directly without parsing later.

```shell
albino build -o hello.bc hello.ws
albino exec hello.bc
```

Tend to be larger than the original source code to byte code. It is a strange thing.

### Disassemble bytecodes

Whitespace's source code is beautiful, but you might be tempted to disassemble due to unavoidable circumstances.
`albino` provides the human readable assembly language that equivalent to the byte code.

```shell
albino gen -o hello.asm hello.bc
```

### Convert to Whitespace

Ook! is orangutan-friendly language, but not secure because can be read by even the orangutan.

First, compile Ook!'s code to bytecode, and then decompile to Whitespace, print it out at the end.
`albino` is very useful for spies.

```shell
albino build hello.ook | albino gen -o hello.ws
```

## License

This project distributed under the MIT License.
https://opensource.org/license/MIT
