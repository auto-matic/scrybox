use anyhow::Result;
use scrybox::{collection::Collection, config, db::ScryboxDB, scryfall_data::ScryfallData};

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        // debug mode should be run with the default config unless desired otherwise
        let _ = std::fs::remove_file("./config.toml");
    }

    print!("Loading config                ");
    let config = config::load_config()?;
    println!("✔");

    print!("Loading collection            ");
    let collection = Collection::load_collection(&config)?;
    println!("✔");

    print!("Loading scryfall data         ");
    let scryfall_data = ScryfallData::load_scryfall_data(&config)?;
    println!("✔");

    if config.db.overwrite_old {
        let _ = std::fs::remove_file("./scrybox.db");
    }

    print!("Loading database connection   ");
    let mut db = ScryboxDB::load_connection(&config)?;
    println!("✔");
    print!("Setting up database schema    ");
    db.setup_db()?;
    println!("✔");
    print!("Inserting collection          ");
    collection.insert_collection(&mut db)?;
    println!("✔");
    print!("Inserting scryfall data       ");
    scryfall_data.insert_data(&mut db)?;
    println!("✔");
    Ok(())
}
