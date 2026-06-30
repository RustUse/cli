/* #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            println!(
                "{{\"command\":\"{}\",\"status\":\"{}\",\"message\":\"{}\"}}",
                escape_json(command),
                escape_json(status),
                escape_json(message)
            );
        } else {
            self.line(message);
        }
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![character],
        })
        .collect()
}
 */

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
            println!(
                "{{\"command\":\"{}\",\"status\":\"{}\",\"message\":\"{}\"}}",
                escape_json(command),
                escape_json(status),
                escape_json(message)
            );
        } else {
            self.line(message);
        }
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![character],
        })
        .collect()
}
