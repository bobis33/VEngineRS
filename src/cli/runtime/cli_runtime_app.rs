use std::collections::HashMap;

pub type CommandFn = fn(&[&str]) -> Result<(), String>;

pub struct RuntimeCli {
    commands: HashMap<String, CommandFn>,
}

impl RuntimeCli {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, func: CommandFn) {
        self.commands.insert(name.to_string(), func);
    }

    pub fn execute(&self, input: &str) -> Result<(), String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let cmd = parts[0];
        let args = &parts[1..];

        match self.commands.get(cmd) {
            Some(f) => f(args),
            None => Err(format!("Unknown command '{}'", cmd)),
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &String> {
        self.commands.keys()
    }
}
