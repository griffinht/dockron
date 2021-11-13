macro_rules! get_first_element {
    ($vector:expr, $message:tt) => {
        {|| {
            if $vector.len() == 0 {
                eprintln!("{}", $message);
                std::process::exit(1);
            }
            return $vector.remove(0);
        }}()
    };
}

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

macro_rules! default_args {
    () => {
        Args {
            n: 1,
            delay: 0,
            verbose: false,
            ignore: false,
            program: String::new(),
            args: Vec::new(),
        };
    }
}

pub fn get_options(options: Options) -> MOptions {
    let mut args = default_args!();

    while args_vec.len() > 0 && args_vec.get(0).unwrap().starts_with("-") {

        macro_rules! parse {
            ($str:tt) => {
                match $str.parse() {
                    Ok(result) => result,
                    Err(error) => {
                        eprintln!("error while parsing -d {} as integer\n{}", $str, error);
                        std::process::exit(1);
                    }
                }
            }
        }

        match arg {
            "n" => {
                let str = get_first_element!(args_vec, "please specify how many times to run with -n <integer>");
                args.n = parse!(str);
            }
            "d" => {
                let str = get_first_element!(args_vec, "please specify delay with -d <integer> (milliseconds)");
                args.delay = parse!(str);
            }
            "v" => {
                args.verbose = true;
            }
            "i" => {
                args.ignore = true;
            }
            _=> {
                eprintln!("unknown argument {}", arg);
                std::process::exit(1);
            }
        }
    }
    args.program = get_first_element!(args_vec, "please specify a program to run or a file to read from");
    return args;
}