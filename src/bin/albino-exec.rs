use std::env;
use std::io::{self, Cursor, Read};
use std::process::exit;

use getopts::{Matches, Options};
use log::debug;
use whitebase::machine;

use albino::command::{LoadCommand, LoadExecutable};

struct CommandBody;

impl LoadExecutable for CommandBody {
    fn handle_error(&self, e: io::Error) {
        println!("{}", e);
        exit(1);
    }

    fn exec<R: Read>(&self, _: &Matches, input: &mut R) {
        let mut buf = Vec::new();
        match input.read_to_end(&mut buf) {
            Ok(_) => {
                let mut reader = Cursor::new(buf);
                let mut machine = machine::with_stdio();
                if let Err(e) = machine.run(&mut reader) {
                    println!("{:?}", e);
                    exit(2);
                }
            }
            Err(e) => self.handle_error(e),
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-exec; args={:?}", env::args_os());

    let mut opts = Options::new();
    let cmd = LoadCommand::new("exec", "[file]", &mut opts, CommandBody);
    cmd.exec();
}
