# flatten

Flatten is a simple CLI tool to combine your codebase into a single Markdown file. It's handy for sharing code with LLMs or for quick overviews.

## Features

- Collects all text/code files from your project
- Respects `.gitignore` and custom include/exclude patterns
- Outputs either a full code dump or a file tree
- Saves to a Markdown file or prints to stdout

## Installation

Install with Cargo:

```sh
cargo install --git https://github.com/brequet/flatten
```

## Usage

```sh
flatten [OPTIONS] [INPUTS]...
```

- `INPUTS` — Files or directories to process (default: current directory)
- `-o, --output-format` — Output format: `full` (default) or `tree`
- `-f, --file <FILE>` — Save output to file (default: `flatten.md`)
- `-p, --print` — Print output to stdout
- `-i, --include <PATTERNS>` — Include file patterns (comma separated, e.g. `*.rs,*.go`)
- `-e, --exclude <PATTERNS>` — Exclude file patterns (comma separated, e.g. `target/*,dist/*`)

### Examples

Flatten the current directory and save to `flatten.md`:

```sh
flatten
```

Flatten a specific folder, print to stdout:

```sh
flatten my_project/ --print
```

Only include Rust and Markdown files:

```sh
flatten -i "*.rs,*.md"
```

Show just the file tree:

```sh
flatten -o tree
```

## License

MIT
