use std::io;
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
}

#[derive(Subcommand, Debug)]
#[command(author, version)]
enum Subcommands {
    /// Verify that the preprocessor supports the wanted output renderer
    Supports {
        #[arg(name="renderer")]
        renderer: String
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
    if let Some(Subcommands::Supports{renderer: _renderer}) = args.command {
        // We support all renderers
        return Ok(());
    }
    
    if let Err(e) = do_preprocessing() {
        eprintln!("Preprocssing failed: {:?}", e);
        Err(e)?
    };
    
    Ok(())
}
