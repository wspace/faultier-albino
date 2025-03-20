use std::env;
use std::io::{self, BufRead, Cursor, Seek, SeekFrom};
use std::process::exit;

use getopts::{Matches, Options};
use log::debug;
use whitebase::machine;
use whitebase::syntax::{Assembly, Brainfuck, Compiler, Ook, Whitespace, DT};

use albino::command::{RunCommand, RunExecutable};
use albino::util::Target;

fn run<B: BufRead, C: Compiler>(buffer: &mut B, syntax: C) {
    let mut bc = Cursor::new(Vec::new());
    match syntax.compile(buffer, &mut bc) {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        _ => {
            bc.seek(SeekFrom::Start(0)).unwrap();
            let mut machine = machine::with_stdio();
            match machine.run(&mut bc) {
                Err(e) => {
                    println!("{:?}", e);
                    exit(2);
                }
                _ => (),
            }
        }
    }
}

struct CommandBody;

impl RunExecutable for CommandBody {
    fn handle_error(&self, e: io::Error) {
        println!("{}", e);
        exit(1);
    }

    fn exec<B: BufRead>(&self, _: &Matches, buffer: &mut B, target: Option<Target>) {
        match target {
            Some(Target::Assembly) => run(buffer, Assembly::new()),
            Some(Target::Brainfuck) => run(buffer, Brainfuck::new()),
            Some(Target::DT) => run(buffer, DT::new()),
            Some(Target::Ook) => run(buffer, Ook::new()),
            Some(Target::Whitespace) => run(buffer, Whitespace::new()),
            None => {
                println!(
                    "syntax should be \"asm\", \"bf\", \"dt\", \"ook\" or \"ws\" (default: ws)"
                );
                exit(1);
            }
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-run; args={:?}", env::args_os());

    let mut opts = Options::new();
    let cmd = RunCommand::new("run", "[-s syntax] [file]", &mut opts, CommandBody);
    cmd.exec();
}
