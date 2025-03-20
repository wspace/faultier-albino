use std::env;
use std::io::{self, BufRead, Write};
use std::process::exit;

use getopts::{Matches, Options};
use log::debug;
use whitebase::syntax::{Assembly, Brainfuck, Compiler, Ook, Whitespace, DT};

use albino::command::{BuildCommand, BuildExecutable};
use albino::util::Target;

fn build<B: BufRead, W: Write, C: Compiler>(input: &mut B, output: &mut W, syntax: C) {
    match syntax.compile(input, output) {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        _ => (),
    }
}

struct CommandBody;

impl BuildExecutable for CommandBody {
    fn handle_error(&self, e: io::Error) {
        println!("{}", e);
        exit(1);
    }

    fn exec<B: BufRead, W: Write>(
        &self,
        _: &Matches,
        buffer: &mut B,
        writer: &mut W,
        target: Option<Target>,
    ) {
        match target {
            Some(Target::Assembly) => build(buffer, writer, Assembly::new()),
            Some(Target::Brainfuck) => build(buffer, writer, Brainfuck::new()),
            Some(Target::DT) => build(buffer, writer, DT::new()),
            Some(Target::Ook) => build(buffer, writer, Ook::new()),
            Some(Target::Whitespace) => build(buffer, writer, Whitespace::new()),
            _ => {
                println!(
                    "syntax should be \"asm\", \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)"
                );
                exit(1);
            }
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-build; args={:?}", env::args_os());

    let mut opts = Options::new();
    let cmd = BuildCommand::new(
        "build",
        "[-s syntax] [-o output] [file]",
        &mut opts,
        CommandBody,
    );
    cmd.exec();
}
