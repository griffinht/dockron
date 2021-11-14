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
    loop {
        if raw_args.len() == 0 { break }
        let mut raw_arg = raw_args.get(i)
            .unwrap()
            .chars();
        let arg;
        if raw_arg.nth(0).unwrap() != '-' {
            break
        }
        if raw_arg.nth(1).unwrap() == '-' { // --arg to arg
            arg = raw_arg.nth(2).unwrap();
        } else {
            arg = raw_arg.nth(1).unwrap(); // -arg to arg
        }

        options.push(
            Option {
                name: arg.to_string(),
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