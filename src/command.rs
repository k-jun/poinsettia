use crate::Result;
#[derive(Debug, Clone, Copy)]
pub enum ExecType {
    Get,
    Set,
}

#[derive(Debug)]
pub struct Command {
    pub exec: ExecType,
    args: Vec<String>,
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

        let exec = &args[0];
        match exec.as_ref() {
            "get" if args.len() == 2 => Some(Command {
                exec: ExecType::Get,
                args,
            }),
            "set" if args.len() == 3 => Some(Command {
                exec: ExecType::Set,
                args,
            }),
            _ => None,
        }
    }

    pub fn exec(&self) -> ExecType {
        self.exec
    }

    pub fn args(&self) -> Vec<String> {
        self.args.clone()
    }
}
