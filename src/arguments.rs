pub struct Option {
    pub name: String,
    pub value: std::option::Option<String>,
}
pub struct Arguments {
    pub options: Vec<Option>,
    pub argument: std::option::Option<String>,
}

pub fn parse_arguments(raw_args: Vec<String>) -> Arguments {
    let mut options = Vec::new();
    let mut i = 0;
    while raw_args.len() > i {
        let mut raw_arg = raw_args.get(i)
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
        i += 1;
        options.push(
            Option {
                name: arg,
                value:
                if raw_args.len() > i {
                    let value = raw_args.get(i).cloned();
                    i += 1;
                    value
                } else {
                    None
                }
            });
    }
    return Arguments { options,
        argument:
        if raw_args.len() > 0 {
            raw_args.get(i).cloned()
        } else {
            None
        }
    };
}