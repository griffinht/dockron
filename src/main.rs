fn main() {
    let mut n = 1; // amount of times to run
    let mut delay = 0; // delay for runs except for the first
    let mut verbose = false; // verbose logging to stderr
    let mut ignore = false; // ignore non zero exit codes

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // first arg is irrelevant?
    let program = args.remove(0);
    while args.get(0).unwrap().starts_with("-") {
        let arg = args
            .remove(0) // pop --arg
            .chars()
            .nth(1)// trim to arg
            .unwrap()
            .to_string();
        let arg = arg.as_str();
        match arg {
            "n" => {
                n = args.remove(0).parse().unwrap();
            }
            "d" => {
                delay = args.remove(0).parse().unwrap();
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

    let n = n;
    let delay = delay;
    let verbose = verbose;
    let ignore = ignore;

    if verbose { eprintln!("Running {} {} times with a {}ms delay with arguments {:?}", program, n, delay, args); }
    let mut command = std::process::Command::new(program.as_str()); // program name
    let command = command.args(args);
    let mut i = 1;
    loop {
        // negative n should run infinitely
        if n >= 0 && i > n { break }
        if verbose { eprintln!("Running {}... ({}/{})", program, i, n) }
        let mut result = command.spawn().unwrap();

        let status = result
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
