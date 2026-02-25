BEGIN;
CREATE TABLE collection_card (
    collection_id integer PRIMARY KEY,
    scryfall_id text,
    quantity integer,
    binder_name text,
    binder_type text
);
CREATE TABLE scryfall_card (
    scrybox_id integer PRIMARY KEY,
    scryfall_id text,
    oracle_id text,
    name text,
    image_jpg text,
    image_png text,
    mana_cost text,
    type_line text,
    oracle_text text,
    power text,
    toughness text,
    standard text,
    modern text,
    pauper text,
    commander text,
    set_id text,
    set_short text,
    set_name text,
    rarity text,
    flavor_text text,
    edhrec_rank integer,
    price text
);
CREATE TABLE color (
    scrybox_id integer,
    color text,
    PRIMARY KEY (scrybox_id, color)
);
CREATE TABLE color_identity (
    scrybox_id integer,
    color text,
    PRIMARY KEY (scrybox_id, color)
);
CREATE TABLE oracle_word (
    scrybox_id integer,
    word text,
    PRIMARY KEY (scrybox_id, word)
);
CREATE TABLE keyword (
    scrybox_id integer,
    keyword text,
    PRIMARY KEY (scrybox_id, keyword)
);
COMMIT;

