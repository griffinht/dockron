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

struct Args {
    n: i32, // amount of times to run
    delay: u64, // delay for runs except for the first
    verbose: bool, // verbose logging to stderr
    ignore: bool, // ignore non zero exit codes
    program: String,
    args: Vec<String>,
}
impl std::fmt::Display for Args {
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

fn get_args_from_file(_file: std::fs::File) -> Args {
    let env_args = Vec::new();

    return get_args_from_env(env_args)
}
fn get_args_from_args(env_args: std::env::Args) -> Args {
    let mut args: Vec<String> = env_args.collect();
    args.remove(0); // first arg is irrelevant?
    return get_args_from_env(args);
}
fn get_args_from_env(mut args_vec: Vec<String>) -> Args {
    let mut args = default_args!();

    while args_vec.len() > 0 && args_vec.get(0).unwrap().starts_with("-") {
        let arg = args_vec
            .remove(0) // pop -arg
            .chars()
            .nth(1)// trim to arg
            .unwrap()
            .to_string();
        let arg = arg.as_str();

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

fn main() {
    let env_args = std::env::args();
    let args: Args;
    match env_args.len() {
        1 => {
            // no args, so look for default file
            match std::fs::File::open(DEFAULT_FILE_NAME) {
                Ok(file) => {
                    args = get_args_from_file(file);
                },
                Err(_error) => { // otherwise just try to run with command line arguments
                    args = get_args_from_args(env_args);
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
            args = get_args_from_file(file);
        }
        _ => {
            // command line args
            args = get_args_from_args(env_args);
        }
    }

    if args.verbose { eprintln!("{}", args); }
    let mut command = std::process::Command::new(args.program.as_str()); // program name
    let command = command.args(args.args);
    let mut i = 1;
    loop {
        // negative n should run infinitely
        if args.n >= 0 && i > args.n { break }
        if args.verbose { eprintln!("Running {}... ({}/{})", args.program, i, args.n) }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                eprintln!("error while spawning program {}\n{}", args.program, error.to_string());
                std::process::exit(1)
            }
        };

        let status = child
            .wait()
            .unwrap();

        let code = status.code().unwrap();
        if args.verbose { eprintln!("{} exited with code {}", args.program, code) }

        if !args.ignore && code != 0 {
            if args.verbose { eprintln!("Non-zero exit code, exiting... (-i to ignore non-zero exit codes)") }
            std::process::exit(code);
        }

        if args.n < 0 || i < args.n {
            if args.verbose { eprintln!("Waiting {}ms for next run...", args.delay) }
            std::thread::sleep(std::time::Duration::from_millis(args.delay));
        }
        i += 1;
    }

    if args.verbose { eprintln!("Finished running {}", args.program) }
}
