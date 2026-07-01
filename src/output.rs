#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Plain,
    Json,
}

#[derive(Clone, Copy, Debug)]
pub struct Output {
    format: OutputFormat,
    quiet: bool,
    verbose: bool,
}

impl Output {
    #[must_use]
    pub const fn new(json: bool, quiet: bool, verbose: bool) -> Self {
        Self {
            format: if json {
                OutputFormat::Json
            } else {
                OutputFormat::Plain
            },
            quiet,
            verbose,
        }
    }

    #[must_use]
    pub const fn is_json(self) -> bool {
        matches!(self.format, OutputFormat::Json)
    }

    pub fn line(self, message: impl AsRef<str>) {
        if !self.quiet {
            println!("{}", message.as_ref());
        }
    }

    pub fn detail(self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet {
            println!("{}", message.as_ref());
        }
    }

    pub fn record(self, command: &str, status: &str, message: &str) {
        if self.is_json() {
            let record = serde_json::json!({
                "command": command,
                "status": status,
                "message": message,
            });

            println!("{record}");
        } else {
            self.line(message);
        }
    }
}
