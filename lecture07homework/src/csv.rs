use std::fmt;

pub struct Csv {
    pub headers: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Determine the maximum width for each column
        // O(T): n
        let mut column_max_widths = self.headers.iter().map(|header| header.len()).collect::<Vec<_>>();
        for row in &self.values {
            for (i, value) in row.iter().enumerate() {
                column_max_widths[i] = column_max_widths[i].max(value.len());
            }
        }

        // Print headers
        for (i, header) in self.headers.iter().enumerate() {
            write!(f, "{:width$} | ", header, width = column_max_widths[i])?;
        }
        writeln!(f)?;

        // Print header separator
        for &width in &column_max_widths {
            for _ in 0..width {
                write!(f, "-")?;
            }
            write!(f, " | ")?;
        }
        writeln!(f)?;

        // Print values
        for row in &self.values {
            for (i, value) in row.iter().enumerate() {
                write!(f, "{:width$} | ", value, width = column_max_widths[i])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

