fn main() {
    let mut n = 1; // amount of times to run
    let mut delay = 0; // delay for runs except for the first
    let mut verbose = false;

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
                n = args.get(0).unwrap().parse().unwrap();
            }
            "d" => {
                delay = args.get(0).unwrap().parse().unwrap();
            }
            "v" => {
                verbose = true;
            }
            _=> {
                eprintln!("unknown argument {}", arg);
                std::process::exit(1);
            }
        }
    }
    if verbose { eprintln!("Running {} {} times with a {}ms delay with arguments {:?}", program, n, delay, args); }
    let mut result = std::process::Command::new(program.as_str()) // program name
        .args(args)// remaining args will be passed to child
        .spawn()
        .unwrap();

    let status = result
        .wait()
        .unwrap();
    let code = status.code().unwrap();
    if verbose { eprintln!("{} exited with code {}", program, code) }
    //todo repeat and delay
    std::process::exit(code);
}
