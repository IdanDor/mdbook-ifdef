#![deny(
    warnings,
    rustdoc::all,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use mdbook_ifdef::IfdefProcessor;

#[derive(Parser, Debug)]
#[command(author, version)]
/// Run as a mdbook preprocesser to ifdef your mdbook!
struct Args {
    #[command(subcommand, name="renderer")]
    command: Option<Subcommands>,

    #[arg(long, short='f')]
    flags_file: Option<PathBuf>,

    #[arg(long, short='e', value_delimiter=',')]
    extra_flags: Vec<String>,
}

#[derive(Subcommand, Debug)]
#[command(author, version)]
enum Subcommands {
    /// Verify that the preprocessor supports the wanted output renderer
    Supports {
        #[arg()]
        renderer: String
    },

    /// Manually execute over specific files, doesn't actually outputs anything, this is merely a parsing test
    Manual {
        #[arg(required=true)]
        target: Vec<PathBuf>
    }
}

fn do_preprocessing(flags: HashSet<String>) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let ifdef = IfdefProcessor::new(flags);
    let new_book = ifdef.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &new_book)?;

    Ok(())
}

fn parse_flags(flags_file: Option<PathBuf>, extra_flags: Vec<String>) -> Result<HashSet<String>, Error> {
    let flags_content = match flags_file {
        Some(path) => fs::read_to_string(path)?,
        None => String::new(),
    };

    Ok(flags_content.split_ascii_whitespace()
        .flat_map(|word| word.split(',')).map(std::borrow::ToOwned::to_owned)
        .chain(extra_flags)
        .collect::<HashSet<_>>()
    )
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let flags = parse_flags(args.flags_file, args.extra_flags)?;

    match args.command {
        Some(Subcommands::Supports{renderer: _renderer}) => {
            // We support all renderers
            Ok(())
        },
        Some(Subcommands::Manual { target }) => {
            {
                use mdbook_ifdef::grammer::FakeMarkdownParser;

                println!("Using flags {:?}", &flags);
                for path in &target {
                    println!("Staring file {path:?}");
                    let string = fs::read_to_string(path)?;
                    println!("Result: {:?}", FakeMarkdownParser::fake_markdown_parse_and_clean(&string, &flags));
                }
            }
            Ok(())
        },
        None => {
            if let Err(e) = do_preprocessing(flags) {
                eprintln!("Preprocssing failed: {e:?}");
                Err(e)?;
            };
            
            Ok(())
        }
    }
}
