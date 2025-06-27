pub mod api_keys;
pub mod backup;
pub mod browser_import;
pub mod crypto;
pub mod document_storage;
pub mod env_connections;
pub mod error;
pub mod generator;
pub mod models;
pub mod notes_config;
pub mod secret_templates;
pub mod secrets;
pub mod secrets_storage;
pub mod ssh_keys;
pub mod storage;
pub mod team_sharing;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}