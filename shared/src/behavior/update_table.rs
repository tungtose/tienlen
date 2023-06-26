use crate::components::Table;

pub fn update_table(card_str: &String, table: &mut Table) {
    *table.cards = card_str.clone();
}
