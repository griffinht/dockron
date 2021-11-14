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
    options.optflag("f", "file", "read arguments from file");
    options.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    let matches = match options.parse(&arguments[1..]) {
        Ok(m) => m,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };

    if matches.opt_present("h") {
        eprint!("{}", options.usage_with_format(|opts| {
            format!(
                "Usage: {} [options...] <command>\n{}\n",
                NAME,
                opts.collect::<Vec<String>>().join("\n")
            )
        }));
        return;
    }
    if matches.opt_present("v") {
        eprintln!("{} version {}", NAME, VERSION);
        return;
    }
    let verbose = matches.opt_present("verbose");
    if matches.opt_present("f") {
        let file = match std::fs::File::open(matches.opt_get::<String>("f").unwrap().unwrap()) {
            Ok(file) => file,
            Err(error) => { panic!("{}", error) }
        };
        eprintln!("{}", file.metadata().unwrap().len());
    } else {
        let mut found_path: Option<std::fs::DirEntry> = None;
        let mut duplicate = false;
        for path in std::fs::read_dir("./").unwrap() {
            let path = path.unwrap();
            if path.file_name().into_string().unwrap().contains(NAME) {
                if found_path.is_some() {
                    duplicate = true;
                    break;
                }

                found_path = Some(path);
            }
        }
        if duplicate {
            if verbose {
                eprintln!("warning: multiple {} files found", NAME);
            }
        } else if found_path.is_some() {
            let found_path = found_path.unwrap();
            eprintln!("found {}", found_path.file_name().into_string().unwrap());
        }
    }
    let n = match matches.opt_get_default("n", 1) {
        Ok(t) => t,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };
    let delay = match matches.opt_get_default("d", 0) {
        Ok(t) => t,
        Err(error) => { eprintln!("{}", error); std::process::exit(1); }
    };
    let ignore = matches.opt_present("i");
    let mut matches = matches;
    let program: String = if matches.free.len() > 0 {
        matches.free.remove(0)
    } else { eprintln!("specify a program to run"); std::process::exit(1); };
    let program = program.as_str();
    let arguments: Vec<String> = matches.free;

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
