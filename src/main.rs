use std::fs;
use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};


use mdbook_censored::CensorProcessor;

#[derive(Parser, Debug)]
#[command(author, version)]
/// Run as a mdbook preprocesser to censor your mdbook!
struct Args {
    #[command(subcommand, name="renderer")]
    command: Option<Subcommands>,

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

fn do_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let censor = CensorProcessor::default();
    let new_book = censor.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &new_book)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Some(Subcommands::Supports{renderer: _renderer}) => {
            // We support all renderers
            Ok(())
        },
        Some(Subcommands::Manual { target }) => {
            {
                use mdbook_censored::grammer::FakeMarkdownParser;
                use mdbook_censored::flags::FlagsHolder;

                println!("Using flags {:?}", &args.extra_flags);
                let flags = FlagsHolder::new(args.extra_flags);
                for path in target.iter() {
                    println!("Staring file {path:?}");
                    let string = fs::read_to_string(path)?;
                    println!("Result: {:?}", FakeMarkdownParser::fake_markdown_parse_and_clean(&string, &flags));
                }
            }
            Ok(())
        },
        None => {
            if let Err(e) = do_preprocessing() {
                eprintln!("Preprocssing failed: {:?}", e);
                Err(e)?
            };
            
            Ok(())
        }
    }
}
