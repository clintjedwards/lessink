mod parser;
mod renderer;

use clap::{Parser, Subcommand};

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
        Commands::Build { input, output } => {}
        Commands::Run { input, output } => {}
    }

    let unparsed_file = std::fs::read_to_string("tests/test.lessink").unwrap();

    let ast = parser::parse(&unparsed_file).unwrap();
    let html = renderer::HtmlRenderer::new();

    println!("{}", html.render(ast));
}

// We walk the input file system and file by file we create the new html file in the output location.
fn build() {}

// This is build followed by a rustembed into a webserver so the user can see what we created.
fn run() {}
