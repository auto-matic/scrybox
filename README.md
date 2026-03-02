# Scrybox

A tool for querying a [ManaBox](https://manabox.app/) collection.

## Setup

1. Export your ManaBox collection by going to the collection tab, tapping the three dots in the top right corner and selecting export
2. Get the current version of the cards data from [Scryfall](https://scryfall.com/docs/api/bulk-data). Make sure to select "All Cards".
3. Place both in the root directory.
4. Adjust the file names in `config.toml`.
5. Start the data ingress by running `cargo run -r --bin data_ingress`.
