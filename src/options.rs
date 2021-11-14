use crate::arguments;

pub struct Options {
    pub n: i32, // amount of times to run
    pub delay: u64, // delay for runs except for the first
    pub verbose: bool, // verbose logging to stderr
    pub ignore: bool, // ignore non zero exit codes
    pub program: String,
    pub args: Vec<String>,
}
impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "n: {}, delay: {}, verbose: {}, ignore: {}, program: {}, args: {:?}", self.n, self.delay, self.verbose, self.ignore, self.program, self.args)
    }
}

macro_rules! default_options {
    () => {
        Options {
            n: 1,
            delay: 0,
            verbose: false,
            ignore: false,
            program: String::new(),
            args: Vec::new(),
        };
    }
}

fn parse<F: std::str::FromStr>(str: String) -> F {
    return match str.parse() {
        Ok(result) => result,
        Err(_error) => {
            eprintln!("error while parsing -d {} as integer\n{}", str, str);//todo print error
            std::process::exit(1);
        }
    }
}

pub fn get_options(arguments: arguments::Arguments) -> Options {
    let mut options = default_options!();

    for option in arguments.options {
        match option.name.as_str() {
            "n" => {
                let value = match option.value {
                    Some(value) => value,
                    None => {
                        eprintln!("please specify how many times to run with -n <integer>");
                        std::process::exit(1)
                    }
                };
                options.n = parse(value);
            }
            "d" => {
                let value = match option.value {
                    Some(value) => value,
                    None => {
                        eprintln!("please specify delay with -d <integer> (milliseconds)");
                        std::process::exit(1)
                    }
                };
                options.delay = parse(value);
            }
            "v" => {
                options.verbose = true;
            }
            "i" => {
                options.ignore = true;
            }
            _=> {
                eprintln!("unknown argument {}", option.name);
                std::process::exit(1);
            }
        }
    }
    options.program = match arguments.argument{
        Some(program) => program,
        None => {
            eprintln!("please specify a program to run or a file to read from");
            std::process::exit(1)
        }
    };
    return options;
}