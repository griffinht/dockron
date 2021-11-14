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
        let arg: &str;
        match raw_arg.next() {
            None => { break; }
            Some(char) => {
                if char != '-' { break }
            }
        }
        // -
        match raw_arg.next() {
            None => {
                eprintln!("specify argument using -?");
                std::process::exit(1);
            }
            Some(char) => {
                if char == '-' // --
                    && raw_arg.next().is_none() { // make sure there is something else
                        eprintln!("specify argument using --???");
                        std::process::exit(1);
                }
                arg = raw_arg.as_str();
            }
        }
        options.push(
            Option {
                name: arg.parse().unwrap(),
                value:
                if raw_args.len() > 0 {
                    i += 1;
                    raw_args.get(i + 1).cloned()
                } else {
                    None
                }
            });
        i += 1;
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