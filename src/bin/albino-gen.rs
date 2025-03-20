use std::env;
use std::io::{self, Cursor, Read, Write};
use std::process::exit;

use getopts::{Matches, Options};
use log::debug;
use whitebase::syntax::{Assembly, Decompiler, Whitespace, DT};

use albino::command::{GenerateCommand, GenerateExecutable};
use albino::util::Target;

fn gen<R: Read, W: Write, D: Decompiler>(input: &mut R, output: &mut W, syntax: D) {
    let mut buf = Vec::new();
    match input.read_to_end(&mut buf) {
        Ok(_) => {
            let mut reader = Cursor::new(buf);
            if let Err(e) = syntax.decompile(&mut reader, output) {
                println!("{}", e);
                exit(1);
            }
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

struct CommandBody;

impl GenerateExecutable for CommandBody {
    fn handle_error(&self, e: io::Error) {
        println!("{}", e);
        exit(1);
    }

    fn exec<R: Read, W: Write>(
        &self,
        _: &Matches,
        reader: &mut R,
        writer: &mut W,
        target: Option<Target>,
    ) {
        match target {
            Some(Target::Assembly) => gen(reader, writer, Assembly::new()),
            Some(Target::DT) => gen(reader, writer, DT::new()),
            Some(Target::Whitespace) => gen(reader, writer, Whitespace::new()),
            _ => {
                println!("syntax should be \"asm\", \"dt\" or \"ws\" (default: ws)");
                exit(1);
            }
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-gen; args={:?}", env::args_os());

    let mut opts = Options::new();
    let cmd = GenerateCommand::new(
        "gen",
        "[-s syntax] [-o output] [file]",
        &mut opts,
        CommandBody,
    );
    cmd.exec();
}
