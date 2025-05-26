use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

pub struct IgnoreHandler {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl IgnoreHandler {
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

        true
    }

    pub fn is_text_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "rs" | "go"
                    | "js"
                    | "ts"
                    | "py"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
                    | "cs"
                    | "php"
                    | "rb"
                    | "swift"
                    | "kt"
                    | "scala"
                    | "clj"
                    | "hs"
                    | "ml"
                    | "fs"
                    | "dart"
                    | "nim"
                    | "zig"
                    | "v"
                    | "odin"
                    | "txt"
                    | "md"
                    | "rst"
                    | "toml"
                    | "yaml"
                    | "yml"
                    | "json"
                    | "xml"
                    | "html"
                    | "css"
                    | "scss"
                    | "sass"
                    | "less"
                    | "sql"
                    | "sh"
                    | "bash"
                    | "zsh"
                    | "fish"
                    | "ps1"
                    | "bat"
                    | "cmd"
                    | "dockerfile"
                    | "gitignore"
                    | "gitattributes"
                    | "editorconfig"
            )
        } else {
            // Check files without extensions (like Dockerfile, Makefile)
            if let Some(filename) = path.file_name() {
                let name = filename.to_string_lossy().to_lowercase();
                matches!(
                    name.as_str(),
                    "dockerfile"
                        | "makefile"
                        | "rakefile"
                        | "gemfile"
                        | "procfile"
                        | "justfile"
                        | "taskfile"
                )
            } else {
                false
            }
        }
    }
}
