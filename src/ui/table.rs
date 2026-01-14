use std::fmt;
use std::fmt::{Display, Write};

use console::StyledObject;

pub struct Table {
    min_widths: Vec<usize>,
    content: Vec<Line>,
}

pub type TableRow = Vec<TableCell>;

enum Line {
    Row(TableRow),
    Separator,
}

#[derive(Clone)]
pub struct TableCell {
    allow_hiding: bool,
    content: String,
    length: usize,
    align: TableAlign,
}

#[derive(Clone)]
pub enum TableAlign {
    Left,
    Center,
    Right,
}

impl Table {
    pub fn new(min_widths: Vec<usize>) -> Self {
        Self {
            min_widths,
            content: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: TableRow) {
        assert!(self.min_widths.len() == row.len());
        self.content.push(Line::Row(row));
    }

    pub fn add_separator(&mut self) {
        self.content.push(Line::Separator);
    }
}

impl TableCell {
    pub fn styled(content: StyledObject<impl Display + Clone>) -> Self {
        let unstyled = content.clone().force_styling(false);
        let length = unstyled.to_string().chars().count();
        Self {
            allow_hiding: false,
            content: content.to_string(),
            length,
            align: TableAlign::Left,
        }
    }

    pub fn optional(content: Option<impl ToString>) -> Self {
        match content {
            Some(content) => content.into(),
            None => TableCell::empty(),
        }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Self {
            allow_hiding: true,
            content: "".into(),
            length: 0,
            align: TableAlign::Left,
        }
    }

    #[allow(unused)]
    pub fn allow_hiding(mut self) -> Self {
        self.allow_hiding = true;
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }
}

impl<T: ToString> From<T> for TableCell {
    fn from(content: T) -> Self {
        let content = content.to_string();
        Self {
            allow_hiding: false,
            length: content.chars().count(),
            content,
            align: TableAlign::Left,
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = self.min_widths.len();
        let mut show = vec![false; cols];
        let mut width = self.min_widths.clone();
        for line in &self.content {
            if let Line::Row(row) = line {
                for (col_show, cell) in show.iter_mut().zip(row.iter()) {
                    *col_show = *col_show || !cell.allow_hiding;
                }
                for (col_width, cell) in width.iter_mut().zip(row.iter()) {
                    *col_width = (*col_width).max(cell.length);
                }
            }
        }
        let first_shown = match show.iter().position(|show| *show) {
            Some(pos) => pos,
            None => return Ok(()),
        };
        let last_shown = show.iter().rposition(|show| *show).unwrap();
        let total_width = width
            .iter()
            .zip(show.iter())
            .map(|(&width, &show)| if show { width + 3 } else { 0 })
            .sum::<usize>()
            .saturating_sub(1);
        let write_row = |f: &mut fmt::Formatter<'_>, row: &TableRow| -> fmt::Result {
            for (i, cell) in row.iter().enumerate().filter(|(i, _)| show[*i]) {
                if i == first_shown {
                    f.write_char(' ')?;
                } else {
                    f.write_str(" | ")?;
                }
                let leftover = width[i] - cell.length;
                let (left_pad, mut right_pad) = match cell.align {
                    TableAlign::Left => (0, leftover),
                    TableAlign::Center => {
                        if width[i] % 2 == 0 {
                            (leftover / 2, (leftover + 1) / 2)
                        } else {
                            ((leftover + 1) / 2, leftover / 2)
                        }
                    }
                    TableAlign::Right => (leftover, 0),
                };
                if i == last_shown {
                    right_pad = 0;
                }
                for _ in 0..left_pad {
                    f.write_char(' ')?;
                }
                f.write_str(&cell.content)?;
                for _ in 0..right_pad {
                    f.write_char(' ')?;
                }
            }
            Ok(())
        };
        for line in &self.content {
            match line {
                Line::Row(row) => write_row(f, row)?,
                Line::Separator => {
                    for _ in 0..total_width {
                        f.write_char('-')?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alignment() {
        let mut table = Table::new(vec![0, 5, 0]);
        table.add_row(vec!["A".into(), "B".into(), "C".into()]);
        table.add_row(vec![
            TableCell::empty(),
            TableCell::from("12").align(TableAlign::Left),
            TableCell::from("12345"),
        ]);
        table.add_row(vec![
            TableCell::empty(),
            TableCell::from("23").align(TableAlign::Center),
            TableCell::from("5").align(TableAlign::Right),
        ]);
        table.add_row(vec![
            TableCell::empty(),
            TableCell::from("45").align(TableAlign::Right),
            TableCell::empty(),
        ]);
        let result = table.to_string();
        assert!(result.contains("| 12    | 12345"));
        assert!(result.contains("|   23  |     5"));
        assert!(result.contains("|    45 |"));
    }

    #[test]
    fn hidden_column() {
        let mut table = Table::new(vec![2, 2]);
        table.add_row(vec![
            TableCell::empty(),
            TableCell::from("hidden").allow_hiding(),
        ]);
        table.add_row(vec!["content".into(), TableCell::empty()]);
        let result = table.to_string();
        assert!(result.contains("content"));
        assert!(!result.contains("hidden"));
    }

    #[test]
    fn consistent_alignment() {
        let mut table = Table::new(vec![3, 4, 5]);
        table.add_row(vec![TableCell::from("1").align(TableAlign::Center); 3]);
        table.add_row(vec![TableCell::from("22").align(TableAlign::Center); 3]);
        table.add_row(vec![TableCell::from("333").align(TableAlign::Center); 3]);
        let result = table.to_string();
        assert!(result.contains("  1  |  1   |   1"));
        assert!(result.contains("  22 |  22  |   22"));
        assert!(result.contains(" 333 | 333  |  333"));
    }
}
