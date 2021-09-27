use std::process::{Command, exit};
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // first arg is irrelevant?
    let mut result = Command::new("./test.sh")
        .args(args)
        .spawn()
        .expect("gsdf");

    let status = result
        .wait()
        .expect("couldt get code");
    exit(status.code().unwrap());
}
