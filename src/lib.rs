/// BetterOffice Sheets: sparse cell grid with formula evaluation stubs.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Formula(String),
}

#[derive(Debug, Default)]
pub struct Sheet {
    cells: HashMap<(u32, u32), CellValue>,
    pub name: String,
}

impl Sheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            cells: HashMap::new(),
        }
    }

    pub fn set(&mut self, row: u32, col: u32, v: CellValue) {
        self.cells.insert((row, col), v);
    }

    pub fn get(&self, row: u32, col: u32) -> &CellValue {
        self.cells.get(&(row, col)).unwrap_or(&CellValue::Empty)
    }

    pub fn sum_column(&self, col: u32, rows: std::ops::Range<u32>) -> f64 {
        rows.filter_map(|r| {
            if let CellValue::Number(n) = self.get(r, col) {
                Some(*n)
            } else {
                None
            }
        })
        .sum()
    }

    pub fn used_cells(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_number_aaa() {
        let mut s = Sheet::new("Sheet1");
        s.set(0, 0, CellValue::Number(42.0));
        if let CellValue::Number(n) = s.get(0, 0) {
            assert_eq!(*n, 42.0);
        } else {
            panic!("wrong type");
        }
    }

    #[test]
    fn test_empty_cell_returns_empty_aaa() {
        let s = Sheet::new("Sheet1");
        assert!(matches!(s.get(99, 99), CellValue::Empty));
    }

    #[test]
    fn test_sum_column_aaa() {
        let mut s = Sheet::new("Sheet1");
        s.set(0, 0, CellValue::Number(10.0));
        s.set(1, 0, CellValue::Number(20.0));
        s.set(2, 0, CellValue::Number(30.0));
        assert_eq!(s.sum_column(0, 0..3), 60.0);
    }

    #[test]
    fn test_used_cells_count_aaa() {
        let mut s = Sheet::new("Sheet1");
        s.set(0, 0, CellValue::Text("hello".into()));
        s.set(1, 1, CellValue::Number(1.0));
        assert_eq!(s.used_cells(), 2);
    }
}
