use std::iter::repeat;
use unicode_width::UnicodeWidthStr;

const TABLE_PADDING: u16 = 3;

struct TableColumn {
    header: String,
    width: u16,
}

impl TableColumn {
    fn new(header: &str) -> Self {
        TableColumn {
            header: header.to_string(),
            width: (char_width(header) as u16) + TABLE_PADDING,
        }
    }

    fn print_header(&self) {
        print_cell(&self.header, self.width);
    }

    fn print_border(&self) {
        print!("{}", padding('-', self.width))
    }
}

#[derive(Default)]
struct TableRow {
    cells: Vec<String>,
}

impl TableRow {
    fn print_cells(&self, columns: &[TableColumn]) {
        for (idx, column) in columns.iter().enumerate() {
            print_cell(&self.cells[idx], column.width);
        }
    }
}

pub struct TableView {
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
}

impl TableView {
    pub fn new(headers: Vec<&str>) -> Self {
        TableView {
            columns: headers.iter().map(|&h| TableColumn::new(h)).collect(),
            rows: vec![],
        }
    }

    pub fn add_row(&mut self, cells: Vec<&str>) {
        let mut row = TableRow::default();

        for (idx, mut column) in self.columns.iter_mut().enumerate() {
            let cell = cells[idx];

            row.cells.push(cell.to_string());

            let len = (char_width(cell) as u16) + TABLE_PADDING;
            if column.width < len {
                column.width = len;
            }
        }

        self.rows.push(row);
    }

    pub fn print_table(&self) {
        for column in &self.columns {
            column.print_header();
        }

        println!();

        for column in &self.columns {
            column.print_border();
        }

        println!();

        for row in &self.rows {
            row.print_cells(self.columns.as_slice());
            println!();
        }

        println!();
    }
}

fn char_width(value: &str) -> usize {
    UnicodeWidthStr::width_cjk(value)
}

fn padding(ch: char, len: u16) -> String {
    repeat(ch).take(len as usize).collect::<String>()
}

fn print_cell(value: &str, width: u16) {
    let count = char_width(value) as u16;
    let right_padding = width - count - TABLE_PADDING;
    print!(" {}{}  ", value, padding(' ', right_padding))
}
