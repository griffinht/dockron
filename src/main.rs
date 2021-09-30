macro_rules! ensure_greater_than_zero {
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

fn main() {
    let mut n = 1; // amount of times to run
    let mut delay = 0; // delay for runs except for the first
    let mut verbose = false; // verbose logging to stderr
    let mut ignore = false; // ignore non zero exit codes

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // first arg is irrelevant?
    while args.len() > 0 && args.get(0).unwrap().starts_with("-") {
        let arg = args
            .remove(0) // pop -arg
            .chars()
            .nth(1)// trim to arg
            .unwrap()
            .to_string();
        let arg = arg.as_str();
        match arg {
            "n" => {
                let str = ensure_greater_than_zero!(args, "please specify how many times to run with -n <integer>");
                n = match str.parse() {
                    Ok(str) => str,
                    Err(error) => {
                        eprintln!("error while parsing -n {} as integer\n{}", str, error);
                        std::process::exit(1);
                    }
                }
            }
            "d" => {
                let str = ensure_greater_than_zero!(args, "please specify delay with -d <integer> (milliseconds)");
                delay = match str.parse() {
                    Ok(str) => str,
                    Err(error) => {
                        eprintln!("error while parsing -d {} as integer\n{}", str, error);
                        std::process::exit(1);
                    }
                }
            }
            "v" => {
                verbose = true;
            }
            "i" => {
                ignore = true;
            }
            _=> {
                eprintln!("unknown argument {}", arg);
                std::process::exit(1);
            }
        }
    }
    let program = ensure_greater_than_zero!(args, "please specify a program to run");
    if verbose { eprintln!("Running {} {} times with a {}ms delay with arguments {:?}", program, n, delay, args); }
    let mut command = std::process::Command::new(program.as_str()); // program name
    let command = command.args(args);
    let mut i = 1;
    loop {
        // negative n should run infinitely
        if n >= 0 && i > n { break }
        if verbose { eprintln!("Running {}... ({}/{})", program, i, n) }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                eprintln!("error while spawning program {}\n{}", program, error.to_string());
                std::process::exit(1)
            }
        };

        let status = child
            .wait()
            .unwrap();

        let code = status.code().unwrap();
        if verbose { eprintln!("{} exited with code {}", program, code) }

        if !ignore && code != 0 {
            if verbose { eprintln!("Non-zero exit code, exiting... (-i to ignore non-zero exit codes)") }
            std::process::exit(code);
        }

        if n < 0 || i < n {
            if verbose { eprintln!("Waiting {}ms for next run...", delay) }
            std::thread::sleep(std::time::Duration::from_millis(delay));
        }
        i += 1;
    }

    if verbose { eprintln!("Finished running {}", program) }
}
