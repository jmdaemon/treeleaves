pub mod app;
pub mod cfgfile;
pub mod config;
pub mod consts;
pub mod data;
pub mod database;
pub mod models;
pub mod plugins;
pub mod schema;
pub mod table;

// Re-export schemas
pub mod schemas {
    #[cfg(feature = "postgres")]
    pub use super::schema::postgres::{audio::*, files::*, images::*, main::*, mime_types::*,videos::*};
}
