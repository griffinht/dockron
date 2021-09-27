fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // first arg is irrelevant?
    let mut result = std::process::Command::new(args.remove(0))
        .args(args)
        .spawn()
        .unwrap();

    let status = result
        .wait()
        .unwrap();
    std::process::exit(status.code().unwrap());
}
