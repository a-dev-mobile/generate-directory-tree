use std::collections::HashMap;

pub struct ArgumentParser {
    pub arguments: HashMap<String, String>,
}

impl ArgumentParser {
    pub fn parse(args: &[String]) -> Self {
        let mut arguments = HashMap::new();
        let mut last_key: Option<String> = None;

        for arg in args {
            if arg.starts_with("--") {
                last_key = Some(arg[2..].to_string());
                arguments.insert(last_key.clone().unwrap(), String::new());
            } else if let Some(key) = last_key.clone() {
                arguments.insert(key, arg.clone());
                last_key = None;
            }
        }

        ArgumentParser { arguments }
    }

    pub fn is_valid(&self) -> Result<(), String> {
        if !self.arguments.contains_key("scanDir") || self.arguments["scanDir"].is_empty() {
            return Err(
                "The '--scanDir' argument is required and must specify an existing directory."
                    .to_string(),
            );
        }

        Ok(())
    }

    pub fn scan_dir(&self) -> &String {
        &self.arguments["scanDir"]
    }

    pub fn show_contents(&self) -> bool {
        self.arguments.contains_key("showContents")
    }

    pub fn exclude_items(&self) -> Vec<String> {
        self.arguments.get("exclude").map_or_else(
            || vec![],
            |v| {
                v.split('|')
                    .map(|item| item.trim()) 
                    .map(String::from)
                    .collect()
            },
        )
    }
}
