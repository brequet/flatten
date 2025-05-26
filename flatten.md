# Flattened Codebase

Total files: 8

## Table of Contents

1. [.\Cargo.toml](#file-1)
2. [.\readme.md](#file-2)
3. [.\src\cli.rs](#file-3)
4. [.\src\error.rs](#file-4)
5. [.\src\ignore_handler.rs](#file-5)
6. [.\src\main.rs](#file-6)
7. [.\src\output.rs](#file-7)
8. [.\src\processor.rs](#file-8)

## File 1: .\Cargo.toml

```toml
[package]
name = "flatten"
version = "0.1.0"
edition = "2024"
authors = ["brequet"]
description = "Flatten a codebase into a single file for LLM consumption"

[[bin]]
name = "flatten"
path = "src/main.rs"

[dependencies]
anyhow = "1.0.98"
clap = { version = "4.5.38", features = ["derive"] }
glob = "0.3.2"
ignore = "0.4.23"
thiserror = "2.0.12"
walkdir = "2.5.0"
```

## File 2: .\readme.md

```md

```

## File 3: .\src\cli.rs

```rs
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
```

## File 4: .\src\error.rs

```rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlattenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pattern error: {0}")]
    Pattern(String),

    #[error("Processing error: {0}")]
    Processing(String),
}

impl From<glob::PatternError> for FlattenError {
    fn from(err: glob::PatternError) -> Self {
        FlattenError::Pattern(err.to_string())
    }
}
```

## File 5: .\src\ignore_handler.rs

```rs
use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

pub struct IgnoreHandler {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl IgnoreHandler {
    const TEXT_EXTENSIONS: &'static [&'static str] = &[
        "rs",
        "go",
        "js",
        "ts",
        "py",
        "java",
        "cpp",
        "c",
        "h",
        "hpp",
        "cs",
        "php",
        "rb",
        "swift",
        "kt",
        "scala",
        "clj",
        "hs",
        "ml",
        "fs",
        "dart",
        "nim",
        "zig",
        "v",
        "odin",
        "txt",
        "md",
        "rst",
        "toml",
        "yaml",
        "yml",
        "json",
        "xml",
        "html",
        "css",
        "scss",
        "sass",
        "less",
        "sql",
        "sh",
        "bash",
        "zsh",
        "fish",
        "ps1",
        "bat",
        "cmd",
        "dockerfile",
        "gitignore",
        "gitattributes",
        "editorconfig",
    ];

    const TEXT_FILENAMES: &'static [&'static str] = &[
        "dockerfile",
        "makefile",
        "rakefile",
        "gemfile",
        "procfile",
        "justfile",
        "taskfile",
    ];

    pub fn new(include_patterns: Vec<String>, exclude_patterns: Vec<String>) -> Self {
        Self {
            include_patterns,
            exclude_patterns,
        }
    }

    pub fn walk_files<P: AsRef<Path>>(&self, path: P) -> impl Iterator<Item = DirEntry> {
        let mut builder = WalkBuilder::new(&path);

        // Enable standard ignore files (.gitignore, .ignore, etc.)
        builder
            .standard_filters(true)
            .ignore(true)
            .git_ignore(true)
            .git_exclude(true)
            .hidden(false);

        // The ignore crate will handle .ignore files automatically
        // Custom exclude patterns are handled in should_include_file()
        builder.build().filter_map(|entry| entry.ok())
    }

    pub fn should_include_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // If include patterns are specified, file must match at least one
        if !self.include_patterns.is_empty() {
            let matches_include = self.include_patterns.iter().any(|pattern| {
                glob::Pattern::new(pattern)
                    .map(|p| p.matches(&path_str))
                    .unwrap_or(false)
            });
            if !matches_include {
                return false;
            }
        }

        // Check if file matches any exclude pattern
        for pattern in &self.exclude_patterns {
            if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
                if glob_pattern.matches(&path_str) {
                    return false;
                }
            }
        }

        // Check if is a flatten.md file
        if Self::is_flatten_md_file(path) {
            return false;
        }

        true
    }

    pub fn is_text_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            Self::TEXT_EXTENSIONS.contains(&ext.as_str())
        } else if let Some(filename) = path.file_name() {
            let name = filename.to_string_lossy().to_lowercase();
            Self::TEXT_FILENAMES.contains(&name.as_str())
        } else {
            false
        }
    }

    fn is_flatten_md_file(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name == "flatten.md")
    }
}
```

## File 6: .\src\main.rs

```rs
use clap::Parser;

mod cli;
mod error;
mod ignore_handler;
mod output;
mod processor;

use cli::Cli;
use processor::FileProcessor;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let processor = FileProcessor::new(cli)?;
    processor.process()
}
```

## File 7: .\src\output.rs

```rs
use crate::error::FlattenError;
use std::fs;
use std::path::PathBuf;

pub struct OutputFormatter {
    format_type: FormatType,
}

#[derive(Debug)]
enum FormatType {
    Full,
    Tree,
}

impl OutputFormatter {
    pub fn new(format: &str) -> Result<Self, FlattenError> {
        let format_type = match format.to_lowercase().as_str() {
            "full" => FormatType::Full,
            "tree" => FormatType::Tree,
            _ => {
                return Err(FlattenError::Processing(format!(
                    "Unknown format: {}. Use 'full' or 'tree'",
                    format
                )));
            }
        };

        Ok(Self { format_type })
    }

    pub fn format_files(&self, files: &[PathBuf]) -> Result<String, FlattenError> {
        match self.format_type {
            FormatType::Tree => self.format_tree(files),
            FormatType::Full => self.format_full(files),
        }
    }

    fn format_tree(&self, files: &[PathBuf]) -> Result<String, FlattenError> {
        let mut output = String::new();
        output.push_str("# File Tree\n\n");
        output.push_str("```\n");
        // Group files by directory
        let mut tree = std::collections::BTreeMap::new();

        for file in files {
            if let Some(parent) = file.parent() {
                tree.entry(parent.to_path_buf())
                    .or_insert_with(Vec::new)
                    .push(file.file_name().unwrap().to_string_lossy().to_string());
            }
        }

        for (dir, mut files_in_dir) in tree {
            files_in_dir.sort();
            output.push_str(&format!("{}:\n", dir.display()));
            for file in files_in_dir {
                output.push_str(&format!("  {}\n", file));
            }
            output.push('\n');
        }

        output.push_str("```\n");
        Ok(output)
    }

    fn format_full(&self, files: &[PathBuf]) -> Result<String, FlattenError> {
        let mut output = String::new();

        output.push_str("# Flattened Codebase\n\n");
        output.push_str(&format!("Total files: {}\n\n", files.len()));

        // Table of contents
        output.push_str("## Table of Contents\n\n");
        for (index, file) in files.iter().enumerate() {
            output.push_str(&format!(
                "{}. [{}](#file-{})\n",
                index + 1,
                file.display(),
                index + 1
            ));
        }
        output.push('\n');

        // File contents
        for (index, file) in files.iter().enumerate() {
            output.push_str(&format!("## File {}: {}\n\n", index + 1, file.display()));

            match fs::read_to_string(file) {
                Ok(content) => {
                    let extension = file.extension().and_then(|ext| ext.to_str()).unwrap_or("");

                    output.push_str(&format!("```{}\n", extension));
                    output.push_str(&content);
                    if !content.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("```\n\n");
                }
                Err(e) => {
                    output.push_str(&format!("Error reading file: {}\n\n", e));
                }
            }
        }

        Ok(output)
    }
}
```

## File 8: .\src\processor.rs

```rs
use crate::cli::Cli;
use crate::ignore_handler::IgnoreHandler;
use crate::output::OutputFormatter;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileProcessor {
    cli: Cli,
    ignore_handler: IgnoreHandler,
    output_formatter: OutputFormatter,
}

impl FileProcessor {
    pub fn new(cli: Cli) -> anyhow::Result<Self> {
        let include_patterns = cli.get_include_patterns();
        let exclude_patterns = cli.get_exclude_patterns();

        let ignore_handler = IgnoreHandler::new(include_patterns, exclude_patterns);
        let output_formatter = OutputFormatter::new(&cli.output_format)?;

        Ok(Self {
            cli,
            ignore_handler,
            output_formatter,
        })
    }

    pub fn process(&self) -> anyhow::Result<()> {
        if self.cli.get_inputs().is_empty() {
            return Err(anyhow::anyhow!("No input files or directories specified"));
        }

        let mut all_files = Vec::new();

        for input in &self.cli.get_inputs() {
            let path = PathBuf::from(input);

            if path.is_file() {
                if self.ignore_handler.should_include_file(&path)
                    && self.ignore_handler.is_text_file(&path)
                {
                    all_files.push(path);
                }
            } else if path.is_dir() {
                let files = self.collect_files_from_directory(&path)?;
                all_files.extend(files);
            } else {
                eprintln!("Warning: '{}' is not a valid file or directory", input);
            }
        }

        if all_files.is_empty() {
            println!("No files found matching the criteria.");
            return Ok(());
        }

        let output = self.output_formatter.format_files(&all_files)?;
        self.handle_output(output)?;

        Ok(())
    }

    fn collect_files_from_directory(&self, dir_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in self.ignore_handler.walk_files(dir_path) {
            let path = entry.path();

            if path.is_file()
                && self.ignore_handler.should_include_file(path)
                && self.ignore_handler.is_text_file(path)
            {
                files.push(path.to_path_buf());
            }
        }

        files.sort();
        Ok(files)
    }

    fn handle_output(&self, output: String) -> anyhow::Result<()> {
        if self.cli.print {
            print!("{}", output);
            return Ok(());
        }

        let output_file = self.cli.get_output_file();
        fs::write(&output_file, output)?;
        println!("Output written to: {}", output_file);

        Ok(())
    }
}
```

