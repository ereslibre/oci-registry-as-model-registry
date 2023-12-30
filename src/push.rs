use crate::{
    errors::{Error, Result},
    model::Model,
};
use oci_distribution::{
    client::{Client, ClientConfig, ClientProtocol, Config, ImageLayer},
    secrets::RegistryAuth,
    Reference,
};

use std::{collections::HashMap, path::PathBuf};

pub(crate) struct PushReference {
    pub config_url: String,
    pub manifest_url: String,
}

pub(crate) async fn push_model(
    uri: &str,
    name: &str,
    version: &str,
    license: &str,
    base: PathBuf,
    tokenizer: PathBuf,
    adapters: Vec<PathBuf>,
) -> Result<PushReference> {
    let reference = Reference::try_from(uri).unwrap();
    let mut client = Client::new(ClientConfig {
        protocol: ClientProtocol::Http,
        ..ClientConfig::default()
    });
    let mut layers = vec![
        ImageLayer::new(
            std::fs::read(base).map_err(|err| Error::Push(format!("{:?}", err)))?,
            // Example model mime-type
            "application/x-model".to_string(),
            // Some information about the model that can be provided by the user
            Some(HashMap::from([
                ("parameters".to_string(), "7b".to_string()),
                ("quantization".to_string(), "q2_K".to_string()),
                ("version".to_string(), "1.2-alpha".to_string()),
            ])),
        ),
        ImageLayer::new(
            std::fs::read(tokenizer).map_err(|err| Error::Push(format!("{:?}", err)))?,
            // Example tokenizer mime-type
            "application/x-huggingface-tokenizer".to_string(),
            None, /* annotations */
        ),
    ];
    for adapter in &adapters {
        layers.push(ImageLayer::new(
            std::fs::read(adapter).map_err(|err| Error::Push(format!("{:?}", err)))?,
            // Example adapter mime-type
            "application/x-lora-adapter".to_string(),
            // Annotations: information about the adapter that can ba provided by the user
            Some(HashMap::from([
                (
                    "description".to_string(),
                    "Specialized for custom codebase PRODUCT-ID-EXAMPLE".to_string(),
                ),
                (
                    "version".to_string(),
                    "1.0-alpha+a2bb697bb485e0dcb483b98f5f6f031cfa21e2e2".to_string(),
                ),
            ])),
        ));
    }
    let result = client
        .push(
            &reference,
            &layers,
            Config::new(
                // Some global manifest that can detail metadata about the model
                serde_json::to_string(&Model {
                    name: name.to_string(),
                    version: version.to_string(),
                    license: license.to_string(),
                })
                .map_err(|err| Error::Push(format!("{:?}", err)))?
                .into_bytes(),
                // Example global model mime-type
                "application/x-quantized-model-with-lora".to_string(),
                // Annotations: information about the model that can ba provided by the user
                Some(HashMap::from([
                    ("use".to_string(), "coding-companion".to_string()),
                    (
                        "maintainer".to_string(),
                        "Team <team@company.com>".to_string(),
                    ),
                ])),
            ),
            &RegistryAuth::Anonymous,
            None,
        )
        .await;
    match result {
        Ok(response) => Ok(PushReference {
            config_url: response.config_url,
            manifest_url: response.manifest_url,
        }),
        Err(err) => Err(Error::Push(err.to_string())),
    }
}
