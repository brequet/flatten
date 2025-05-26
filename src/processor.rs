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
        if self.cli.inputs.is_empty() {
            return Err(anyhow::anyhow!("No input files or directories specified"));
        }

        let mut all_files = Vec::new();

        for input in &self.cli.inputs {
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
        if let Some(output_file) = self.cli.get_output_file() {
            fs::write(&output_file, output)?;
            println!("Output written to: {}", output_file);
        } else if self.cli.print {
            print!("{}", output);
        } else {
            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(output.clone()) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                        print!("{}", output);
                    } else {
                        println!("Output copied to clipboard.");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to access clipboard: {}", e);
                    print!("{}", output);
                }
            }
        }

        Ok(())
    }
}
