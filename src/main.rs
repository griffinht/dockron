mod arguments;
mod options;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // first arg is irrelevant?
    let arguments = arguments::parse_arguments(args);

    let options = options::get_options(arguments);

    /*match env_args.len() {
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
    }*/

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
