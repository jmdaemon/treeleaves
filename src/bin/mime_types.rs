use std::fs;

use diesel::prelude::*;
use indexmap::IndexMap;
use serde_json::Value;

//use treeleaves::{*, schema::postgres::mime_types};
use treeleaves::models::*;

fn main() {
    use treeleaves::schema::postgres::mime_types::mime_types::dsl::*;
    // Load the data file
    const DB_MIME_JSON: &str = "data/db.json";
    let conts = fs::read_to_string(DB_MIME_JSON)
        .expect("Could not find db.json");
    let data: IndexMap<&str, Value> = serde_json::from_str(&conts)
        .expect("Could not deserialize data");

    // Load the database
    //dotenv().ok();
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //const MIME_DATA_URL: &str = ".env/mime_types";
    //let database_url = fs::read_to_string(MIME_DATA_URL)
    //let database_url = fs::read_to_string("postgresql://localhost:5450")
    //let database_url = fs::read_to_string("postgres://postgres@localhost:5450/mime_types")
    //let database_url = fs::read_to_string("postgresql://postgres@localhost:5450/mime_types")

    //let database_url = "postgresql://postgres@localhost:5450/mime_types";
    //let database_url = "postgresql://localhost:5450/mime_types";

    //let database_url = "postgresql://postgres@localhost:5450";
    let database_url = "postgresql://postgres@localhost:5450";

        //.expect("Could not read database url");
    
    //let con = SqliteConnection::establish(&database_url)
        //.unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    //let con = PgConnection::establish(&database_url)
    //let mut con = PgConnection::establish(&database_url)
    let mut con = PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    /*
    // Retrieving Results
    let results = mime_types
        .filter(mime_type.eq("text"))
        .limit(5)
        .select(MIMEType::as_select())
        .load(&mut con)
        .expect("Error loading posts");

    println!("Displaying {} mime_types", results.len());
    for res in results {
        println!("{}", res.id);
        println!("-----------\n");
        println!("{}", res.mime_type);
    }
    */

    // Serialize to the database
    
    let mut media_types = vec![];
    let mut index = 1;
    for (k, _) in data {
        //let mime_type =  MIMEType { id: index, mime_type: v.as_str().to_owned() };
        let media_type =  MIMEType { id: index, mime_type: k.to_string() };
        media_types.push(media_type);
        index += 1;
    }

    diesel::insert_into(mime_types)
        .values(media_types)
        .execute(&mut con)
        .expect("Could not fill mime_types table");
}
