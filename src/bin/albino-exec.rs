use std::io::{self, Cursor, Read};
use std::os;

use getopts::{Matches, Options};
use log::debug;
use whitebase::machine;

use albino::command::{LoadCommand, LoadExecutable};

struct CommandBody;

impl LoadExecutable for CommandBody {
    fn handle_error(&self, e: io::Error) {
        println!("{}", e);
        os::set_exit_status(1);
    }

    fn exec<R: Read>(&self, _: &Matches, input: &mut R) {
        let mut buf = Vec::new();
        match input.read_to_end(&mut buf) {
            Ok(_) => {
                let mut reader = Cursor::new(buf);
                let mut machine = machine::with_stdio();
                match machine.run(&mut reader) {
                    Err(e) => {
                        println!("{}", e);
                        os::set_exit_status(2);
                    }
                    _ => (),
                }
            }
            Err(e) => self.handle_error(e),
        }
    }
}

fn main() {
    debug!("executing; cmd=albino-exec; args={}", os::args());

    let mut opts = Options::new();
    let cmd = LoadCommand::new("exec", "[file]", &mut opts, CommandBody);
    cmd.exec();
}
