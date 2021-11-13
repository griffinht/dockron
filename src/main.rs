const DEFAULT_FILE_NAME: &str = "dockron";

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

struct Options {
    n: i32, // amount of times to run
    delay: u64, // delay for runs except for the first
    verbose: bool, // verbose logging to stderr
    ignore: bool, // ignore non zero exit codes
    program: String,
    args: Vec<String>,
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

fn get_options(options: Options) -> MOptions {
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

struct Option {
    name: String,
    value: std::option<String>,
}
struct Argument {
    options: Vec<Option>,
    argument: std::option<String>,
}

fn parse_args(mut raw_args: Vec<String>) -> Argument {
    let mut options = Vec::new();
    loop {
        if raw_args.len() == 0 { break }
        let mut raw_arg = raw_args
            .remove(0)
            .chars();
        let arg;
        if raw_arg[0] != '-' {
            break
        }
        if raw_arg[1] == '-' { // --arg to arg
            arg = raw_arg.nth(2);
        } else {
            arg = raw_arg.nth(1); // -arg to arg
        }

        options.push(
            Option {
                name: arg.unwrap().to_string(),
                value:
                if raw_args.len() > 0 {
                    value = Some(raw_args.remove(0))
                } else {
                    value = None
                }
            });
    }
    return Argument { options,
        argument:
        if raw_args.len() > 0 {
            Some(raw_args.remove(0))
        } else {
            None
        }
    };
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // first arg is irrelevant?
    let args = parse_args(args);

    let options: Options;
    match env_args.len() {
        1 => {
            // no args, so look for default file
            match std::fs::File::open(DEFAULT_FILE_NAME) {
                Ok(file) => {
                    options = get_args_from_file(file);
                },
                Err(_error) => { // otherwise just try to run with command line arguments
                    options = get_args_from_args(env_args);
                }
            };
        }
        2 => {
            // dockron file specified, so look for that
            let mut env_args = env_args;
            let path = env_args.nth(1).unwrap();
            let file = match std::fs::File::open(path.as_str()) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("error while opening file {}\n{}", path, error);
                    std::process::exit(1)
                }
            };
            options = get_args_from_file(file);
        }
        _ => {
            // command line args
            options = get_args_from_args(env_args);
        }
    }

    if options.verbose { eprintln!("{}", options); }
    let mut command = std::process::Command::new(options.program.as_str()); // program name
    let command = command.args(options.args);
    let mut i = 1;
    loop {
        // negative n should run infinitely
        if options.n >= 0 && i > options.n { break }
        if options.verbose { eprintln!("Running {}... ({}/{})", options.program, i, options.n) }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                eprintln!("error while spawning program {}\n{}", options.program, error.to_string());
                std::process::exit(1)
            }
        };

        let status = child
            .wait()
            .unwrap();

        let code = status.code().unwrap();
        if options.verbose { eprintln!("{} exited with code {}", options.program, code) }

        if !options.ignore && code != 0 {
            if options.verbose { eprintln!("Non-zero exit code, exiting... (-i to ignore non-zero exit codes)") }
            std::process::exit(code);
        }

        if options.n < 0 || i < options.n {
            if options.verbose { eprintln!("Waiting {}ms for next run...", options.delay) }
            std::thread::sleep(std::time::Duration::from_millis(options.delay));
        }
        i += 1;
    }

    if options.verbose { eprintln!("Finished running {}", options.program) }
}
