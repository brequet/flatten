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
                    "Unknown format: {format}. Use 'full' or 'tree'"
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
                output.push_str(&format!("  {file}\n"));
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

                    output.push_str(&format!("```{extension}\n"));
                    output.push_str(&content);
                    if !content.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("```\n\n");
                }
                Err(e) => {
                    output.push_str(&format!("Error reading file: {e}\n\n"));
                }
            }
        }

        Ok(output)
    }
}
