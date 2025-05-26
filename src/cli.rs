use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "flatten", version = "0.1.0")]
#[clap(about = "Flatten a codebase into a single file for LLM consumption")]
pub struct Cli {
    /// Files or directories to process
    pub inputs: Vec<String>,

    /// Output format: tree or full (default: full)
    #[clap(short = 'o', long, default_value = "full")]
    pub output_format: String,

    /// Save to file. If no filename provided, saves to flatten.md
    #[clap(short = 'f', long, value_name = "FILE")]
    pub file: Option<String>,

    /// Print output to stdout instead of copying to clipboard
    #[clap(short, long)]
    pub print: bool,

    /// Include file patterns (comma separated, e.g., "*.rs,*.go")
    #[clap(short, long)]
    pub include: Option<String>,

    /// Exclude file patterns (comma separated, e.g., "target/*,dist/*")
    #[clap(short, long)]
    pub exclude: Option<String>,
}

impl Cli {
    pub fn get_output_file(&self) -> Option<String> {
        let result = match &self.file {
            Some(filename) => Some(filename.clone()),
            None => Some("flatten.md".to_string()),
        };
        result
    }

    pub fn get_include_patterns(&self) -> Vec<String> {
        self.include
            .as_ref()
            .map(|patterns| patterns.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_exclude_patterns(&self) -> Vec<String> {
        self.exclude
            .as_ref()
            .map(|patterns| patterns.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
}
