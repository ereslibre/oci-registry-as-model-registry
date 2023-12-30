use crate::{
    cache::{self, AdapterData, ModelData, TokenizerData},
    errors::{Error, Result},
    model::ModelPaths,
};
use oci_distribution::{
    client::{Client, ClientConfig, ClientProtocol},
    secrets::RegistryAuth,
    Reference,
};
use std::path::PathBuf;

pub(crate) async fn pull_model(uri: &str, output: Option<PathBuf>) -> Result<ModelPaths> {
    let reference = Reference::try_from(uri).unwrap();
    let mut client = Client::new(ClientConfig {
        protocol: ClientProtocol::Http,
        ..ClientConfig::default()
    });
    let image_data = client
        .pull(
            &reference,
            &RegistryAuth::Anonymous,
            vec![
                "application/x-model",
                "application/x-huggingface-tokenizer",
                "application/x-lora-adapter",
            ],
        )
        .await
        .map_err(|err| Error::Pull(format!("{:?}", err)))?;

    let mut model = None;
    let mut tokenizer = None;
    let mut adapters = Vec::new();
    for layer in image_data.layers {
        match layer.media_type.as_str() {
            "application/x-model" => {
                model = Some((
                    layer.data,
                    serde_json::to_string(&layer.annotations)
                        .map_err(|err| Error::Pull(format!("{:?}", err)))?,
                ));
            }
            "application/x-huggingface-tokenizer" => {
                tokenizer = Some((
                    layer.data,
                    serde_json::to_string(&layer.annotations)
                        .map_err(|err| Error::Pull(format!("{:?}", err)))?,
                ));
            }
            "application/x-lora-adapter" => adapters.push(AdapterData {
                data: layer.data,
                annotations: serde_json::to_string(&layer.annotations)
                    .map_err(|err| Error::Pull(format!("{:?}", err)))?,
            }),
            media_type => eprintln!("unknown media type {}", media_type),
        }
    }

    if let (Some((model, model_annotations)), Some((tokenizer, tokenizer_annotations))) =
        (model, tokenizer)
    {
        return cache::save_model(
            uri,
            output,
            image_data.config.data,
            ModelData {
                data: model,
                annotations: model_annotations,
            },
            TokenizerData {
                data: tokenizer,
                annotations: tokenizer_annotations,
            },
            adapters,
        );
    }

    Err(Error::Pull("error pulling model".to_string()))
}
