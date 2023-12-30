#[macro_use]
extern crate lazy_static;

mod cache;
mod describe;
mod errors;
mod list;
mod model;
mod pull;
mod push;
mod run;

use clap::{Parser, Subcommand};
use errors::{Error, Result};
use push::PushReference;
use std::{io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Describe {
        uri: String,
    },
    List,
    Pull {
        uri: String,
        output: Option<PathBuf>,
    },
    Push {
        uri: String,
        name: String,
        version: String,
        license: String,
        base: PathBuf,
        tokenizer: PathBuf,
        adapters: Vec<PathBuf>,
    },
    Run {
        uri: String,
    },
    RunRaw {
        model_path: PathBuf,
        tokenizer_path: PathBuf,
        adapter_path: Vec<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Describe { uri } => {
            describe::show_image_information(uri).await?;
            Ok(())
        }
        Command::List => list::list_models(),
        Command::Pull { uri, output } => {
            pull::pull_model(uri, output.clone()).await?;
            Ok(())
        }
        Command::Push {
            uri,
            name,
            version,
            license,
            base,
            tokenizer,
            adapters,
        } => {
            let PushReference {
                config_url,
                manifest_url,
            } = push::push_model(
                uri,
                name,
                version,
                license,
                base.clone(),
                tokenizer.clone(),
                adapters.clone(),
            )
            .await?;
            println!(
                "model pushed successfully; config_url: {config_url}; manifest_url: {manifest_url}"
            );
            Ok(())
        }
        Command::Run { uri } => {
            let mut prompt = Vec::new();
            std::io::stdin().read_to_end(&mut prompt).unwrap();

            let model = pull::pull_model(uri, None).await?;

            run::run_local(
                &model.model_path,
                &model.tokenizer_path.expect("tokenizer missing"),
                model.adapter_paths,
                &String::from_utf8(prompt).expect("invalid prompt"),
            )
            .map_err(|err| Error::Run(err.to_string()))?;

            Ok(())
        }
        Command::RunRaw {
            model_path,
            tokenizer_path,
            adapter_path,
        } => {
            let mut prompt = String::new();
            std::io::stdin().read_line(&mut prompt).unwrap();

            run::run_local(model_path, tokenizer_path, adapter_path.clone(), &prompt)
                .map_err(|err| Error::Run(err.to_string()))?;

            Ok(())
        }
    }
}
