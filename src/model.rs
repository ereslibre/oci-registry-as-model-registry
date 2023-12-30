use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Model {
    pub name: String,
    pub version: String,
    pub license: String,
}

pub(crate) struct ModelPaths {
    pub model_path: PathBuf,
    pub tokenizer_path: Option<PathBuf>,
    pub adapter_paths: Vec<PathBuf>,
}

impl TryFrom<Vec<u8>> for Model {
    type Error = Error;

    fn try_from(model: Vec<u8>) -> std::result::Result<Self, Error> {
        serde_json::from_str::<Model>(
            std::str::from_utf8(&model).map_err(|err| Error::Store(format!("{:?}", err)))?,
        )
        .map_err(|err| Error::Store(format!("{:?}", err)))
    }
}
