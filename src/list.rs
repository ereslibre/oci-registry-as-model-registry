use crate::{
    cache::CONFIG,
    errors::{Error, Result},
    model::Model,
};

use prettytable::{row, Table};

pub(crate) fn list_models() -> Result<()> {
    let data_file = CONFIG
        .place_data_file("db")
        .map_err(|err| Error::Store(format!("{:?}", err)))?;
    let tree = sled::open(data_file).map_err(|err| Error::Store(format!("{:?}", err)))?;

    let mut table = Table::new();

    table.add_row(row!["URI", "Name", "Version", "License"]);

    for (key, value) in tree.iter().flatten() {
        let uri = std::str::from_utf8(&key).map_err(|err| Error::Store(format!("{:?}", err)))?;
        let model = serde_json::from_str::<Model>(
            std::str::from_utf8(&value).map_err(|err| Error::Store(format!("{:?}", err)))?,
        )
        .map_err(|err| Error::Store(format!("{:?}", err)))?;
        table.add_row(row![uri, model.name, model.version, model.license]);
    }

    table.printstd();

    Ok(())
}
