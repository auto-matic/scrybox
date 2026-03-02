use std::{fs, io, str::FromStr};

use crate::{config::Config, db::ScryboxDB};
use anyhow::Result;
use csv::Reader;
use rayon::prelude::*;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CollectionCard {
    binder_name: String,
    binder_type: String,
    quantity: i32,
    scryfall_id: String,
}

#[derive(Debug)]
pub struct Collection {
    cards: Vec<CollectionCard>,
}

impl Collection {
    pub fn size(&self) -> usize {
        self.cards.len()
    }
    pub fn load_collection(config: &Config) -> Result<Collection> {
        let collection_str = fs::read_to_string(&config.files.collection)?
            .replace("Binder Name", "binder_name")
            .replace("Binder Type", "binder_type")
            .replace("Quantity", "quantity")
            .replace("Scryfall ID", "scryfall_id");
        let mut rdr = Reader::from_reader(collection_str.as_bytes());
        Ok(Collection {
            cards: rdr
                .deserialize()
                .par_bridge()
                .filter_map(|r| r.ok())
                .collect(),
        })
    }
    /*
    pub fn load_collection(config: &Config) -> Result<Collection> {
        let r = Regex::new(r"(?m)^(?P<binder_name>[^,]+),(?P<binder_type>[^,]+),[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,[^,]+,(?P<quantity>[^,]+),[^,]+,(?P<scryfall_id>[^, ]+),").unwrap(); // Compilation of regex has to work
        let collection_str = fs::read_to_string(&config.files.collection)?;

        let caps = r.captures_iter(&collection_str);
        let _caps = r.captures_iter(&collection_str);
        let caps_len: i32 = _caps.map(|_| 1).sum();
        let str_len: i32 = collection_str.lines().map(|_| 1).sum();
        println!("Lines: {}\n Captures: {}", str_len, caps_len);
        let cards: Vec<CollectionCard> = caps
            .par_bridge()
            .map(|m| {
                Ok(CollectionCard {
                    binder_name: m["binder_name"].into(),
                    binder_type: m["binder_type"].into(),
                    quantity: m["quantity"].parse()?,
                    scryfall_id: m["scryfall_id"].into(),
                })
            })
            .filter_map(|c: Result<CollectionCard, <i32 as FromStr>::Err>| c.ok())
            .collect();
        Ok(Collection { cards })
    }*/

    pub fn insert_collection(&self, db: &mut ScryboxDB) -> Result<()> {
        let tx = db.connection.transaction()?;
        {
            let mut stmt = tx.prepare("INSERT INTO collection_card (binder_name, binder_type, quantity, scryfall_id, collection_id) VALUES (?1, ?2, ?3, ?4, ?5)")?;
            for (i, c) in self.cards.iter().enumerate() {
                stmt.execute((
                    &c.binder_name,
                    &c.binder_type,
                    &c.quantity,
                    &c.scryfall_id,
                    i as i32,
                ))?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}
