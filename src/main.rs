extern crate getopts;

const NAME: &str = "dockron";
const VERSION: &str = "1.0";

fn main() {
    let arguments: Vec<String>;

    arguments = std::env::args().collect();

    let mut options = getopts::Options::new();

    options.optflag("h", "help", "print help");
    options.optflag("v", "version", "print version");
    options.optopt("n", "", "how many times to run", "<integer>");
    options.optopt("d", "delay", "delay of each run except for the first", "<milliseconds>");
    options.optflag("", "verbose", "verbose output, useful for debugging");
    options.optflag("i", "ignore", "ignore non-zero exit codes and keep running");
    let matches = match options.parse(&arguments[1..]) {
        Ok(m) => m,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };

    if matches.opt_present("h") {
        eprint!("{}", options.usage_with_format(|opts| {
            format!(
                "Usage: dockron [options...] <command>\n{}\n",
                opts.collect::<Vec<String>>().join("\n")
            )
        }));
        return;
    }
    if matches.opt_present("v") {
        eprintln!("{} version {}", NAME, VERSION);
        return;
    }
    let verbose = matches.opt_present("v");
    let n = match matches.opt_get_default("n", 1) {
        Ok(t) => t,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };
    let delay = match matches.opt_get_default("d", 0) {
        Ok(t) => t,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };
    let ignore = matches.opt_present("i");
    let program = "echo";
    let arguments: Vec<String> = Vec::new();

    let mut command = std::process::Command::new(program); // program name
    let command = command.args(arguments);
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
