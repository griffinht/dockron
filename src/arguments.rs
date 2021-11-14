pub struct Option {
    pub name: String,
    pub value: std::option::Option<String>,
}
pub struct Arguments {
    pub options: Vec<Option>,
    pub argument: Vec<String>,
}

pub fn parse_arguments(raw_args: Vec<String>) -> Arguments {
    let mut options = Vec::new();
    let mut raw_args = raw_args.iter().peekable();
    loop {
        let raw_arg = raw_args.peek();
        if raw_arg.is_none() { break }
        let mut raw_arg = raw_arg
            .unwrap()
            .chars();
        let arg;
        match raw_arg.next() {
            None => { break; }
            Some(char) => {
                if char != '-' { break }
            }
        }
        // -
        let mut raw_arg = raw_arg.peekable();
        match raw_arg.peek() {
            None => {
                eprintln!("specify argument using -?");
                std::process::exit(1);
            }
            Some(char) => {
                if *char == '-' { // --
                    raw_arg.next();
                    if raw_arg.peek().is_none() { // make sure there is something else
                        eprintln!("specify argument using --???");
                        std::process::exit(1);
                    }
                }

                arg = raw_arg.collect::<String>();
            }
        }
        raw_args.next();
        options.push(
            Option {
                name: arg,
                value:
                if raw_args.peek().is_some() {
                    raw_args.next().cloned()
                } else {
                    None
                }
            });
    }
    return Arguments { options,
        argument:
        if raw_args.peek().is_some() {
            //raw_args.next();
            raw_args.cloned().collect()
        } else {
            Vec::new()
        }
    };
}