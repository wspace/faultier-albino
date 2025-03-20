use std::io::{self, stdin, stdout, BufRead, BufReader, Read, Write};
use std::os;

use getopts::{Matches, Options};

use crate::util::{detect_target, Target};

pub trait Executable {
    fn handle_error(&self, e: io::Error);
    fn exec(&self, m: &Matches);
}

pub struct Command<'a, T> {
    command: &'static str,
    usage: &'static str,
    options: &'a mut Options,
    inner: T,
}

impl<'a, E: Executable> Command<'a, E> {
    pub fn new(
        command: &'static str,
        usage: &'static str,
        options: &'a mut Options,
        exec: E,
    ) -> Command<'a, E> {
        options.optflag("h", "help", "");
        Command {
            command: command,
            usage: usage,
            options: options,
            inner: exec,
        }
    }

    pub fn exec(&self) {
        let matches = match self.options.parse(os::args().tail()) {
            Ok(m) => m,
            Err(e) => {
                println!("{}", e);
                os::set_exit_status(1);
                return;
            }
        };
        if matches.opt_present("h") {
            self.print_usage();
        } else {
            self.inner.exec(&matches);
        }
    }

    fn print_usage(&self) {
        let brief = format!("usage: albino {} {}", self.command, self.usage);
        print!("{}", self.options.usage(&brief));
    }
}

pub trait RunExecutable {
    fn handle_error(&self, e: io::Error);
    fn exec<B: BufRead>(&self, m: &Matches, buffer: &mut B, target: Option<Target>);
}

pub struct RunCommand<T> {
    inner: T,
}

impl<E: RunExecutable> RunCommand<E> {
    pub fn new<'a>(
        command: &'static str,
        usage: &'static str,
        options: &'a mut Options,
        exec: E,
    ) -> Command<'a, RunCommand<E>> {
        options.optopt("s", "syntax", "set input file syntax", "syntax");
        Command::new(command, usage, options, RunCommand { inner: exec })
    }
}

impl<E: RunExecutable> Executable for RunCommand<E> {
    fn handle_error(&self, e: io::Error) {
        self.inner.handle_error(e);
    }

    fn exec(&self, m: &Matches) {
        let syntax = m.opt_str("s");
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(file) => {
                    let mut buffer = BufReader::new(file);
                    self.inner
                        .exec(m, &mut buffer, detect_target(syntax, filename));
                }
                Err(e) => self.inner.handle_error(e),
            }
        } else {
            let stdin = stdin();
            self.inner
                .exec(m, &mut stdin.lock(), detect_target(syntax, &"".to_string()));
        }
    }
}

pub trait BuildExecutable {
    fn handle_error(&self, e: io::Error);
    fn exec<B: BufRead, W: Write>(
        &self,
        m: &Matches,
        buffer: &mut B,
        writer: &mut W,
        target: Option<Target>,
    );
}

pub struct BuildCommand<T> {
    inner: T,
}

impl<E: BuildExecutable> BuildCommand<E> {
    pub fn new<'a>(
        command: &'static str,
        usage: &'static str,
        options: &'a mut Options,
        exec: E,
    ) -> Command<'a, RunCommand<BuildCommand<E>>> {
        options.optopt("o", "", "set output file name", "name");
        RunCommand::new(command, usage, options, BuildCommand { inner: exec })
    }
}

impl<E: BuildExecutable> RunExecutable for BuildCommand<E> {
    fn handle_error(&self, e: io::Error) {
        self.inner.handle_error(e);
    }

    fn exec<B: BufRead>(&self, m: &Matches, buffer: &mut B, target: Option<Target>) {
        match m.opt_str("o") {
            Some(ref name) => match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                Ok(ref mut output) => self.inner.exec(m, buffer, output, target),
                Err(e) => self.inner.handle_error(e),
            },
            None => self.inner.exec(m, buffer, &mut stdout(), target),
        }
    }
}

pub trait LoadExecutable {
    fn handle_error(&self, e: io::Error);
    fn exec<R: Read>(&self, m: &Matches, reader: &mut R);
}

pub struct LoadCommand<T> {
    inner: T,
}

impl<E: LoadExecutable> LoadCommand<E> {
    pub fn new<'a>(
        command: &'static str,
        usage: &'static str,
        options: &'a mut Options,
        exec: E,
    ) -> Command<'a, LoadCommand<E>> {
        Command::new(command, usage, options, LoadCommand { inner: exec })
    }
}

impl<E: LoadExecutable> Executable for LoadCommand<E> {
    fn handle_error(&self, e: io::Error) {
        self.inner.handle_error(e);
    }

    fn exec(&self, m: &Matches) {
        if !m.free.is_empty() {
            let ref filename = m.free[0];
            match File::open(&Path::new(filename.as_slice())) {
                Ok(ref mut file) => self.inner.exec(m, file),
                Err(e) => self.inner.handle_error(e),
            }
        } else {
            self.inner.exec(m, &mut stdin());
        }
    }
}

pub trait GenerateExecutable {
    fn handle_error(&self, e: io::Error);
    fn exec<R: Read, W: Write>(
        &self,
        m: &Matches,
        reader: &mut R,
        writer: &mut W,
        target: Option<Target>,
    );
}

pub struct GenerateCommand<T> {
    inner: T,
}

impl<E: GenerateExecutable> GenerateCommand<E> {
    pub fn new<'a>(
        command: &'static str,
        usage: &'static str,
        options: &'a mut Options,
        exec: E,
    ) -> Command<'a, LoadCommand<GenerateCommand<E>>> {
        options.optopt("s", "syntax", "set input file syntax", "syntax");
        options.optopt("o", "", "set output file name", "name");
        LoadCommand::new(command, usage, options, GenerateCommand { inner: exec })
    }
}

impl<E: GenerateExecutable> LoadExecutable for GenerateCommand<E> {
    fn handle_error(&self, e: io::Error) {
        self.inner.handle_error(e);
    }

    fn exec<R: Read>(&self, m: &Matches, reader: &mut R) {
        let syntax = m.opt_str("s");
        match m.opt_str("o") {
            Some(ref name) => match File::open_mode(&Path::new(name.as_slice()), Open, Write) {
                Ok(ref mut output) => {
                    self.inner
                        .exec(m, reader, output, detect_target(syntax, name))
                }
                Err(e) => self.inner.handle_error(e),
            },
            None => self.inner.exec(
                m,
                reader,
                &mut stdout(),
                detect_target(syntax, &"".to_string()),
            ),
        }
    }
}
