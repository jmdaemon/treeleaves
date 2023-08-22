use crate::{
    data::{metadata, size, get_mime_type, DATA_MIME_JSON, DataMap},
    database::{SharedConnection, TargetConnection},
    schema::postgres,
    models::*,
    batch_insert
};

use std::{fs, path::Path};

use diesel::prelude::*;
use walkdir::WalkDir;
use anyhow::Result;

// Manage Treeleaves database tables
//
// Follow the following format when importing the generated table schemas
//      - Import table [name] as tbl_name
//      - Import field [name] as field_name

pub fn pop_mime_types(con: &mut SharedConnection) {
    let con = &mut con.0;

    // Retrieve our media type data
    let conts = fs::read_to_string(DATA_MIME_JSON)
        .expect("Could not find db.json");
    let data: DataMap = serde_json::from_str(&conts)
    .expect("Could not deserialize data");

    // Serialize it to the database
    let mut mime_types = vec![];
    let mut id = 1;
    for (k, _) in data {
        let media_type =  MIMEType { id, mime_type: k.to_string() };
        mime_types.push(media_type);
        id += 1;
    }
    use postgres::mime_types::mime_types::dsl::mime_types as tbl_mime_types;
    batch_insert!(con, "mime_types", tbl_mime_types, mime_types);
}

pub fn pop_files(con: &mut TargetConnection, dir: &Path) -> Result<()> {
    let con = &mut con.0;
    let mut id = 1;
    let mut files = vec![];
    for file_entry in WalkDir::new(dir) {

        let file_entry = file_entry?;
        let path = file_entry.path();
        let metadata = metadata(path);

        let (name, size, mime_type) = (path.file_name().unwrap(), size(&metadata), get_mime_type(path));

        use postgres::mime_types::mime_types::dsl::{mime_types as tbl_mime_types, mime_type as field_mime_type};
        let mime_type_id = tbl_mime_types
            .filter(field_mime_type.eq(mime_type.to_string()))
            .first::<(i32, String)>(con)?
            .0;

        let file = File { id,
            name: name.to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            size: size.try_into()?, mime_type_id
        };
        files.push(file);
        id += 1;
    }
    use postgres::files::files::table as tbl_files;
    batch_insert!(con, "files", tbl_files, files);
    Ok(())
}
