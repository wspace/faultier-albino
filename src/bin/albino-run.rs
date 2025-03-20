use std::io::{IoError, MemReader, MemWriter};
use std::os;

use getopts::Matches;
use log::debug;
use whitebase::machine;
use whitebase::syntax::{Assembly, Brainfuck, Compiler, Ook, Whitespace, DT};

use albino::command::{RunCommand, RunExecutable};
use albino::util::Target;

fn run<B: Buffer, C: Compiler>(buffer: &mut B, syntax: C) {
    let mut writer = MemWriter::new();
    match syntax.compile(buffer, &mut writer) {
        Err(e) => {
            println!("{}", e);
            os::set_exit_status(1);
        }
        _ => {
            let mut reader = MemReader::new(writer.unwrap());
            let mut machine = machine::with_stdio();
            match machine.run(&mut reader) {
                Err(e) => {
                    println!("{}", e);
                    os::set_exit_status(2);
                }
                _ => (),
            }
        }
    }
}

struct CommandBody;

impl RunExecutable for CommandBody {
    fn handle_error(&self, e: IoError) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<B: Buffer>(&self, _: &Matches, buffer: &mut B, target: Option<Target>) {
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
                os::set_exit_status(1);
            }
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-run; args={}", os::args());

    let mut opts = vec![];
    let cmd = RunCommand::new("run", "[-s syntax] [file]", &mut opts, CommandBody);
    cmd.exec();
}
