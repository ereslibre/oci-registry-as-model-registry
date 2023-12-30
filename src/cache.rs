use crate::{
    errors::{Error, Result},
    model::{Model, ModelPaths},
};
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

lazy_static! {
    pub static ref CONFIG: BaseDirectories =
        BaseDirectories::with_prefix("oci-registry-as-model-registry")
            .expect("error accessing base directory for user configuration");
}

fn data_path(output: &Option<PathBuf>, asset: &Path) -> Result<PathBuf> {
    match output {
        Some(root_path) => {
            std::fs::create_dir_all(root_path.join("adapters"))
                .map_err(|err| Error::Store(format!("{:?}", err)))?;
            Ok(root_path.join(asset))
        }
        None => Ok(CONFIG
            .place_data_file(asset)
            .map_err(|err| Error::Store(format!("{:?}", err)))?),
    }
}

pub(crate) struct ModelData {
    pub data: Vec<u8>,
    pub annotations: String,
}

pub(crate) struct TokenizerData {
    pub data: Vec<u8>,
    pub annotations: String,
}

pub(crate) struct AdapterData {
    pub data: Vec<u8>,
    pub annotations: String,
}

pub(crate) fn save_model(
    uri: &str,
    output: Option<PathBuf>,
    config: Vec<u8>,
    model: ModelData,
    tokenizer: TokenizerData,
    adapters: Vec<AdapterData>,
) -> Result<ModelPaths> {
    let uri_digest = format!("{:x}", Sha1::digest(uri));

    let model_path = data_path(&output, &Path::new(&uri_digest).join("model"))?;
    std::fs::write(&model_path, &model.data).map_err(|err| Error::Store(format!("{}", err)))?;

    let model_metadata_path = data_path(&output, &Path::new(&uri_digest).join("model.json"))?;
    std::fs::write(&model_metadata_path, &model.annotations)
        .map_err(|err| Error::Store(format!("{}", err)))?;

    let tokenizer_path = data_path(&output, &Path::new(&uri_digest).join("tokenizer"))?;
    std::fs::write(&tokenizer_path, &tokenizer.data)
        .map_err(|err| Error::Store(format!("{}", err)))?;

    let tokenizer_metadata_path =
        data_path(&output, &Path::new(&uri_digest).join("tokenizer.json"))?;
    std::fs::write(&tokenizer_metadata_path, &tokenizer.annotations)
        .map_err(|err| Error::Store(format!("{}", err)))?;

    let mut adapter_paths = Vec::new();
    for (
        i,
        AdapterData {
            ref data,
            ref annotations,
        },
    ) in adapters.iter().enumerate()
    {
        let adapter_path = data_path(
            &output,
            &Path::new(&uri_digest)
                .join("adapters")
                .join(format!("{}", i)),
        )?;
        adapter_paths.push(adapter_path.clone());
        std::fs::write(&adapter_path, data).map_err(|err| Error::Store(format!("{}", err)))?;

        let adapter_metadata_path = data_path(
            &output,
            &Path::new(&uri_digest)
                .join("adapters")
                .join(format!("{}.json", i)),
        )?;
        std::fs::write(&adapter_metadata_path, annotations)
            .map_err(|err| Error::Store(format!("{}", err)))?;
    }

    let data_file = CONFIG
        .place_data_file("db")
        .map_err(|err| Error::Store(format!("{:?}", err)))?;
    let tree = sled::open(data_file).map_err(|err| Error::Store(format!("{:?}", err)))?;

    tree.insert(
        uri,
        serde_json::to_string(
            &Model::try_from(config).map_err(|err| Error::Store(format!("{:?}", err)))?,
        )
        .map_err(|err| Error::Store(format!("{:?}", err)))?
        .as_bytes()
        .to_vec(),
    )
    .map_err(|err| Error::Store(format!("{}", err)))?;

    Ok(ModelPaths {
        model_path,
        tokenizer_path: Some(tokenizer_path),
        adapter_paths,
    })
}
