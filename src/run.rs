use core::result::Result;
use llm::Model;

use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub(crate) fn run_local(
    model_path: &Path,
    tokenizer_path: &Path,
    adapter_paths: Vec<PathBuf>,
    prompt: &str,
) -> Result<(), llm::LoadError> {
    let llama = llm::load::<llm::models::Llama>(
        model_path,
        llm::TokenizerSource::HuggingFaceTokenizerFile(tokenizer_path.to_path_buf()),
        llm::ModelParameters {
            // Add as many LORA adapters as desired
            lora_adapters: if adapter_paths.is_empty() {
                None
            } else {
                Some(adapter_paths)
            },
            ..Default::default()
        },
        llm::load_progress_callback_stdout,
    )?;
    let mut session = llama.start_session(Default::default());
    let res = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: llm::Prompt::Text(prompt),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(120),
        },
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    Ok(())
}
