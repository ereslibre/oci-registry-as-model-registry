use crate::errors::{Error, Result};
use oci_distribution::{
    client::{Client, ClientConfig, ClientProtocol},
    secrets::RegistryAuth,
    Reference,
};

pub(crate) async fn show_image_information(uri: &str) -> Result<()> {
    let reference = Reference::try_from(uri).unwrap();
    let mut client = Client::new(ClientConfig {
        protocol: ClientProtocol::Http,
        ..ClientConfig::default()
    });
    let manifest_and_config = client
        .pull_manifest_and_config(&reference, &RegistryAuth::Anonymous)
        .await;
    match manifest_and_config {
        Ok((manifest, hash, manifest_config)) => {
            println!("all good");
            println!("  manifest: {}", manifest);
            println!("  hash: {}", hash);
            println!("  manifest config: {}", manifest_config);
            Ok(())
        }
        Err(err) => Err(Error::Describe(err.to_string())),
    }
}
