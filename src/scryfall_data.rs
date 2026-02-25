use anyhow::Result;
use rusqlite::params;
use serde::Deserialize;

use crate::{config::Config, db::ScryboxDB};

#[derive(Debug)]
pub struct ScryfallData {
    cards: Vec<ScryfallCard>,
}

impl ScryfallData {
    pub fn load_scryfall_data(config: &Config) -> Result<ScryfallData> {
        let file = std::fs::File::open(&config.files.scryfall_data)?;
        let reader = std::io::BufReader::new(file);
        let cards: Vec<ScryfallCard> = serde_json::from_reader(reader)?;
        Ok(ScryfallData { cards })
    }

    pub fn insert_data(&self, db: &mut ScryboxDB) -> Result<()> {
        let tx = db.connection.transaction()?;
        {
            let mut insert_card = tx.prepare("INSERT INTO scryfall_card (scrybox_id, scryfall_id, oracle_id, name, image_jpg, image_png, mana_cost, type_line, oracle_text, power, toughness, standard, modern, pauper, commander, set_id, set_short, set_name, rarity, flavor_text, edhrec_rank, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)")?;
            let mut insert_color =
                tx.prepare("INSERT INTO color (scrybox_id, color) VALUES (?1, ?2)")?;
            let mut insert_color_identity =
                tx.prepare("INSERT INTO color_identity (scrybox_id, color) VALUES (?1, ?2)")?;
            let mut insert_oracle_word =
                tx.prepare("INSERT OR IGNORE INTO oracle_word (scrybox_id, word) VALUES (?1, ?2)")?;
            let mut insert_keyword =
                tx.prepare("INSERT INTO keyword (scrybox_id, keyword) VALUES (?1, ?2)")?;
            for (i, card) in self.cards.iter().enumerate() {
                let scrybox_id = i as isize; // cast needed because usize is not supported in sql
                insert_card.execute(params![
                    scrybox_id,
                    &card.id,
                    &card.oracle_id,
                    &card.name,
                    &card.image_uris.as_ref().map(|img| { img.normal.clone() }),
                    &card.image_uris.as_ref().map(|img| { img.png.clone() }),
                    &card.mana_cost,
                    &card.type_line,
                    &card.oracle_text,
                    &card.power,
                    &card.toughness,
                    &card.legalities.standard,
                    &card.legalities.modern,
                    &card.legalities.pauper,
                    &card.legalities.commander,
                    &card.set_id,
                    &card.set,
                    &card.set_name,
                    &card.rarity,
                    &card.flavor_text,
                    &card.edhrec_rank,
                    &card.prices.eur,
                ])?;
                if let Some(colors) = &card.colors {
                    for color in colors {
                        insert_color.execute(params![scrybox_id, color])?;
                    }
                }
                for color in &card.color_identity {
                    insert_color_identity.execute(params![scrybox_id, color])?;
                }
                if let Some(oracle_text) = &card.oracle_text {
                    let oracle_text = oracle_text
                        .replace(".", "")
                        .replace(",", "")
                        .replace("(", "")
                        .replace(")", "")
                        .replace("\"", "")
                        .replace("\"", "")
                        .replace(":", "")
                        .to_lowercase();
                    let oracle_words = oracle_text.split_whitespace(); // TODO: implement more filtering
                    for word in oracle_words {
                        insert_oracle_word.execute(params![scrybox_id, word])?;
                    }
                }
                for word in &card.keywords {
                    insert_keyword.execute(params![scrybox_id, word])?;
                }
            }
        }
        tx.commit()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ScryfallCard {
    id: String,
    oracle_id: String,
    name: String,
    image_uris: Option<ImageURIs>,
    mana_cost: Option<String>,
    type_line: String,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    keywords: Vec<String>,
    legalities: Legalities,
    set_id: String,
    set: String,
    set_name: String,
    rarity: String,
    flavor_text: Option<String>,
    edhrec_rank: Option<i32>,
    prices: Prices,
}

#[derive(Debug, Deserialize)]
pub struct ImageURIs {
    normal: String,
    png: String,
}

#[derive(Debug, Deserialize)]
pub struct Legalities {
    standard: String,
    modern: String,
    pauper: String,
    commander: String,
}

#[derive(Debug, Deserialize)]
pub struct Prices {
    eur: Option<String>,
}
