mod parser;
mod renderer;

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Debug, Parser, Clone)]
#[command(name = "lessink")]
#[command(bin_name = "lessink")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Build {
        input: std::path::PathBuf,
        output: std::path::PathBuf,
    },

    Run {
        input: std::path::PathBuf,
        output: std::path::PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build { input, output } => build(input, output),
        Commands::Run { input, output } => {}
    }
}

// We walk the input file system and file by file we create the new html file in the output location.
fn build(input: std::path::PathBuf, output: std::path::PathBuf) {
    walk_and_render(&input, &output);
}

// This is build followed by a rustembed into a webserver so the user can see what we created.
fn run() {}

struct Structure {
    kind: StructureKind,
    inner: Vec<Structure>,
}

enum StructureKind {
    Section(String),
    NewLevelStart,
    File(String, String), // file name -> file path
}

fn read_settings(input_dir: &std::path::Path) -> Vec<(String, String)> {
    let mut settings_file_path = input_dir.to_path_buf();
    settings_file_path.push("lessink_settings.lessink");

    let settings_file = std::fs::read_to_string(settings_file_path).unwrap();

    let ast = parser::parse(&settings_file).unwrap();
    let mut structure = vec![];

    for node in ast {
        match node.as_rule() {
            parser::Rule::h1 => {
                let text = node.into_inner().next().unwrap().as_span().as_str();
                structure.push(StructureKind::Section(text.to_string()));
            }
            parser::Rule::link => {}
            parser::Rule::list => {}
            _ => continue,
        }
    }

    todo!();
}

fn walk_and_render(input: &std::path::Path, output: &std::path::Path) {
    for entry in WalkDir::new(input) {
        let entry = entry.unwrap();
        let path = entry.path();

        let rel_path = path.strip_prefix(input).unwrap();
        let out_path = output.join(rel_path);

        if path.is_dir() {
            std::fs::create_dir_all(&out_path).unwrap();
        } else if path.is_file() {
            if path.file_name().unwrap().to_string_lossy() == "lessink_settings.lessink" {
                continue;
            }

            let unparsed_file = std::fs::read_to_string(path).expect("test");

            let ast = parser::parse(&unparsed_file).unwrap();
            let html = renderer::Html::render(ast);

            std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();
            std::fs::write(&out_path, html).unwrap();
        }
    }
}
