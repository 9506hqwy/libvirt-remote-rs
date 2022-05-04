use super::util;
use std::string::ToString;

const KV_SPACING: u16 = 2;

#[derive(Default)]
pub struct KeyValueView {
    rows: Vec<(String, String)>,
    key_width: u16,
}

impl KeyValueView {
    pub fn add_row(&mut self, key: &str, value: impl ToString) {
        let len = (util::char_width(key) as u16) + KV_SPACING;
        if self.key_width < len {
            self.key_width = len;
        }

        self.rows.push((key.to_string(), value.to_string()));
    }

    pub fn print_kv(&self) {
        for row in &self.rows {
            print_row(&row.0, &row.1, self.key_width);
            println!();
        }

        println!();
    }
}

pub fn print_row(key: &str, value: &str, width: u16) {
    let count = util::char_width(key) as u16;
    let right_padding = width - count - KV_SPACING;
    print!("{}{}  {}", key, util::padding(' ', right_padding), value)
}
