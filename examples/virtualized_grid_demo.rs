//! Virtualized Grid Demo
//!
//! Demonstrates the VirtualizedGrid widget with:
//! - 1,000 rows × 20 columns (20,000 cells total!)
//! - Cell widget recycling for efficient scrolling
//! - Fixed header row
//! - Optional row headers
//! - Horizontal and vertical scrolling
//! - CSV-like table display

use assorted_widgets::application::Application;
use assorted_widgets::layout::Style;
use assorted_widgets::paint::Color;
use assorted_widgets::widget::Widget;
use assorted_widgets::widgets::{GridDataSource, Label, VirtualizedGrid};

/// CSV-like data source with rows and columns
struct CsvDataSource {
    row_count: usize,
    col_count: usize,
    column_headers: Vec<String>,
}

impl CsvDataSource {
    fn new(rows: usize, cols: usize) -> Self {
        // Generate column headers (A, B, C, ..., AA, AB, ...)
        let column_headers = (0..cols)
            .map(|i| {
                if i < 26 {
                    ((b'A' + i as u8) as char).to_string()
                } else {
                    let first = ((b'A' + (i / 26 - 1) as u8) as char).to_string();
                    let second = ((b'A' + (i % 26) as u8) as char).to_string();
                    format!("{}{}", first, second)
                }
            })
            .collect();

        Self {
            row_count: rows,
            col_count: cols,
            column_headers,
        }
    }

    /// Generate cell content (simulating data)
    fn cell_value(&self, row: usize, col: usize) -> String {
        // Simulate different types of data
        match col % 5 {
            0 => format!("Row {}", row + 1),
            1 => format!("${:.2}", (row * col) as f64 / 10.0),
            2 => format!("{}", row * col),
            3 => {
                let names = ["Alice", "Bob", "Charlie", "David", "Eve"];
                names[row % names.len()].to_string()
            }
            4 => {
                if (row + col) % 2 == 0 {
                    "✓".to_string()
                } else {
                    "✗".to_string()
                }
            }
            _ => format!("R{}C{}", row + 1, col + 1),
        }
    }
}

impl GridDataSource for CsvDataSource {
    fn row_count(&self) -> usize {
        self.row_count
    }

    fn column_count(&self) -> usize {
        self.col_count
    }

    fn column_header(&self, col: usize) -> String {
        self.column_headers[col].clone()
    }

    fn column_width(&self, _col: usize) -> f64 {
        120.0  // Fixed width for all columns
    }

    fn row_height(&self, _row: usize) -> Option<f64> {
        Some(32.0)  // Fixed height for all rows
    }

    fn cell_widget(
        &mut self,
        row: usize,
        col: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        let text = self.cell_value(row, col);

        // Try to reuse the widget
        if let Some(mut widget) = reused_widget {
            if widget.as_any().downcast_ref::<Label>().is_some() {
                // It's a Label, reuse it
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.set_text(&text);
                }
                return widget;
            }
        }

        // Create new Label
        Box::new(
            Label::new(&text)
                .font_size(14.0)
                .text_color(Color::rgb(0.2, 0.2, 0.2))
        )
    }

    fn has_row_headers(&self) -> bool {
        true  // Show row numbers
    }

    fn row_header_widget(
        &mut self,
        row: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        let text = format!("{}", row + 1);

        // Try to reuse
        if let Some(mut widget) = reused_widget {
            if widget.as_any().downcast_ref::<Label>().is_some() {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.set_text(&text);
                }
                return widget;
            }
        }

        // Create new
        Box::new(
            Label::new(&text)
                .font_size(12.0)
                .text_color(Color::rgb(0.4, 0.4, 0.4))
        )
    }

    fn row_header_width(&self) -> f64 {
        60.0
    }

    fn header_widget(
        &mut self,
        col: usize,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        let text = self.column_header(col);

        // Try to reuse
        if let Some(mut widget) = reused_widget {
            if widget.as_any().downcast_ref::<Label>().is_some() {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.set_text(&text);
                }
                return widget;
            }
        }

        // Create new
        Box::new(
            Label::new(&text)
                .font_size(14.0)
                .text_color(Color::rgb(0.3, 0.3, 0.3))
        )
    }
}

fn main() {
    eprintln!("Starting Virtualized Grid Demo...");

    Application::launch(|app| {
        eprintln!("Application launched, spawning window...");

        app.spawn_window("Virtualized Grid Demo - 1,000 × 20 Table", 800.0, 600.0, |window| {
            eprintln!("Window created, setting up grid...");

            // Create a virtualized grid with 1,000 rows × 20 columns
            let grid = VirtualizedGrid::new()
                .data_source(Box::new(CsvDataSource::new(1_000, 20)))
                .default_row_height(32.0)
                .header_height(40.0)
                .background(Color::WHITE)
                .header_background(Color::rgb(0.93, 0.93, 0.93))
                .show_grid_lines(true)
                .grid_line_color(Color::rgb(0.85, 0.85, 0.85))
                .layout_style(Style {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    ..Style::default()
                });

            window.set_main_widget(grid);

            eprintln!("Grid configured with 1,000 rows × 20 columns = 20,000 cells!");
            eprintln!("  - Fixed header row with column names (A, B, C, ...)");
            eprintln!("  - Row headers showing row numbers (1, 2, 3, ...)");
            eprintln!("  - Horizontal and vertical scrolling");
            eprintln!("  - Try scrolling! Only visible cells are created.");
        });

        eprintln!("Window spawned successfully!");
    });
}
