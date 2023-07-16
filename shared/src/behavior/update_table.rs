use crate::components::Table;

pub fn update_table(card_str: &str, table: &mut Table) {
    *table.cards = card_str.to_owned();
}
