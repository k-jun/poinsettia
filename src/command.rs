#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

impl Command {
    pub fn parse(raw: String) -> Option<Command> {
        let args = raw
            .split_whitespace()
            .map(|x| x.to_lowercase())
            .collect::<Vec<String>>();
        if args.len() <= 1 {
            return None;
        }

        match (args[0].as_ref(), args.len()) {
            ("get", 2) => Some(Command::Get {
                key: args[1].to_string(),
            }),
            ("set", 3) => Some(Command::Set {
                key: args[1].to_string(),
                value: args[2].to_string(),
            }),
            (_, _) => None,
        }
    }
}

#[test]
fn test_parse() {
    struct TestCase {
        in_str: String,
        out_command: Option<Command>,
    }

    let cases = vec![
        TestCase {
            in_str: "get ipsa".to_string(),
            out_command: Some(Command::Get {
                key: "ipsa".to_string(),
            }),
        },
        TestCase {
            in_str: "set voluptatum dolor".to_string(),
            out_command: Some(Command::Set {
                key: "voluptatum".to_string(),
                value: "dolor".to_string(),
            }),
        },
        TestCase {
            in_str: "invalid command".to_string(),
            out_command: None,
        },
    ];

    for c in cases {
        let command = Command::parse(c.in_str);
        assert_eq!(command, c.out_command);
    }
}
