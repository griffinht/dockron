struct Option {
    name: String,
    value: std::option<String>,
}
struct Argument {
    options: Vec<Option>,
    argument: std::option<String>,
}

pub fn parse_args(mut raw_args: Vec<String>) -> Argument {
    let mut options = Vec::new();
    loop {
        if raw_args.len() == 0 { break }
        let mut raw_arg = raw_args
            .remove(0)
            .chars();
        let arg;
        if raw_arg[0] != '-' {
            break
        }
        if raw_arg[1] == '-' { // --arg to arg
            arg = raw_arg.nth(2);
        } else {
            arg = raw_arg.nth(1); // -arg to arg
        }

        options.push(
            Option {
                name: arg.unwrap().to_string(),
                value:
                if raw_args.len() > 0 {
                    value = Some(raw_args.remove(0))
                } else {
                    value = None
                }
            });
    }
    return Argument { options,
        argument:
        if raw_args.len() > 0 {
            Some(raw_args.remove(0))
        } else {
            None
        }
    };
}