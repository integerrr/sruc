use crate::formatter::string_formatter::{self, StringAlignment};
use std::slice::Iter;

pub struct DiscordTable<T: Clone> {
    columns: Vec<DiscordTableColumn<T>>,
    data_rows: Vec<T>,
}

impl<T: Clone> DiscordTable<T> {
    pub fn new() -> DiscordTable<T> {
        DiscordTable {
            columns: vec![],
            data_rows: vec![],
        }
    }

    pub fn add_column(&mut self, col: DiscordTableColumn<T>) {
        self.columns.push(col);
    }

    pub fn add_data_rows(&mut self, rows: &[T]) {
        rows.iter().for_each(|r| self.data_rows.push(r.clone()));
    }

    pub fn columns(&self) -> Iter<'_, DiscordTableColumn<T>> {
        self.columns.iter()
    }

    pub fn data_rows(&self) -> Iter<'_, T> {
        self.data_rows.iter()
    }

    pub fn get_table_header(&self) -> String {
        let col_names = self
            .columns
            .iter()
            .map(|c| string_formatter::align(c.name.clone(), c.width, c.alignment))
            .collect::<Vec<_>>();

        format!("`{}`\n", col_names.join("|"))
    }

    pub fn get_table_body(&self) -> String {
        let mut body = String::new();
        for row in self.data_rows.as_slice() {
            let cols_in_a_row = self
                .columns
                .iter()
                .map(|c| string_formatter::align((c.column_fn)(row.clone()), c.width, c.alignment))
                .collect::<Vec<_>>();

            body += &format!("{}\n", cols_in_a_row.join("|"));
        }

        body
    }
}

impl<T: Clone> Default for DiscordTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DiscordTableColumn<T> {
    name: String,
    column_fn: Box<dyn Fn(T) -> String>,
    width: usize,
    alignment: StringAlignment,
}

impl<T> DiscordTableColumn<T> {
    pub fn new(
        name: impl Into<String>,
        column_fn: impl Fn(T) -> String + Clone + 'static,
        width: usize,
        alignment: StringAlignment,
    ) -> DiscordTableColumn<T> {
        DiscordTableColumn {
            name: name.into(),
            column_fn: Box::new(column_fn.clone()),
            width,
            alignment,
        }
    }
}
