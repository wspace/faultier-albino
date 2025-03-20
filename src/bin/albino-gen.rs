use std::io::{IoError, MemReader};
use std::os;

use getopts::{Matches, Options};
use log::debug;
use whitebase::syntax::{Assembly, Decompiler, Whitespace, DT};

use albino::command::{GenerateCommand, GenerateExecutable};
use albino::util::Target;

fn gen<R: Reader, W: Writer, D: Decompiler>(input: &mut R, output: &mut W, syntax: D) {
    match input.read_to_end() {
        Ok(buf) => {
            let mut reader = MemReader::new(buf);
            match syntax.decompile(&mut reader, output) {
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(1);
                }
                _ => (),
            }
        }
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
    }
}

struct CommandBody;

impl GenerateExecutable for CommandBody {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<R: Reader, W: Writer>(
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
                os::set_exit_status(1);
            }
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-gen; args={}", os::args());

    let mut opts = Options::new();
    let cmd = GenerateCommand::new(
        "gen",
        "[-s syntax] [-o output] [file]",
        &mut opts,
        CommandBody,
    );
    cmd.exec();
}
