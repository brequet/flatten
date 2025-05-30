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

    /// Print output to stdout
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
    pub fn get_inputs(&self) -> Vec<String> {
        if self.inputs.is_empty() {
            vec![".".to_string()]
        } else {
            self.inputs.clone()
        }
    }

    pub fn get_output_file(&self) -> String {
        self.file.as_deref().unwrap_or("flatten.md").to_string()
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
