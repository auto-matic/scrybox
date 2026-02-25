use anyhow::Result;
use rusqlite::{Connection, OpenFlags};

use crate::config::Config;

#[derive(Debug)]
pub struct ScryboxDB {
    pub connection: Connection,
}

impl ScryboxDB {
    pub fn load_connection(config: &Config) -> Result<ScryboxDB> {
        let of = OpenFlags::default().union(OpenFlags::SQLITE_OPEN_EXRESCODE);
        let connection = Connection::open_with_flags(&config.files.db, of)?;
        Ok(ScryboxDB { connection })
    }

    pub fn setup_db(&self) -> Result<()> {
        self.connection
            .execute_batch(include_str!("./sql/setup.sql"))?;
        Ok(())
    }
}
