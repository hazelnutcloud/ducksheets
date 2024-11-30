use std::collections::BTreeMap;

use eframe::egui::{self, ViewportBuilder};

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "DuckSheets",
        options,
        Box::new(|_cc| Ok(Box::<DuckSheetsApp>::default())),
    )
}

struct DuckSheetsApp {
    sheets: BTreeMap<usize, Sheet>,
    // num_sheets: usize,
}

impl Default for DuckSheetsApp {
    fn default() -> Self {
        Self {
            sheets: BTreeMap::new(),
            // num_sheets: 0,
        }
    }
}

struct Sheet {
    num_rows: u64,
    num_columns: usize,
    default_column: egui_table::Column,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            num_rows: 1000,
            num_columns: 27,
            default_column: egui_table::Column::new(100.0).resizable(true),
        }
    }
}

impl Sheet {
    fn idx_to_column_letter(n: usize) -> String {
        let mut result = String::new();
        let mut n = n + 1;

        while n > 0 {
            n -= 1;
            let c = (b'A' + (n % 26) as u8) as char;
            result.insert(0, c);
            n /= 26;
        }

        result
    }
}

impl egui_table::TableDelegate for Sheet {
    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::HeaderCellInfo) {
        let egui::Rect { min, max } = ui.max_rect();
        ui.painter().hline(
            min.x..=max.x,
            max.y,
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(32.0, 0.0))
            .show(ui, |ui| {
                if cell.col_range.start > 0 {
                    ui.label(Self::idx_to_column_letter(cell.group_index - 1));
                }
            });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
        let egui::Rect { min, max } = ui.max_rect();
        ui.painter().hline(
            min.x..=max.x,
            max.y,
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(
                if cell.col_nr == 0 { 8.0 } else { 32.0 },
                0.0,
            ))
            .show(ui, |ui| {
                if cell.col_nr == 0 {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label({ cell.row_nr + 1 }.to_string());
                    });
                }
            });
    }

    fn default_row_height(&self) -> f32 {
        32.0
    }
}

impl eframe::App for DuckSheetsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let sheet = self.sheets.entry(0).or_insert_with(Default::default);

            let table = egui_table::Table::new()
                .columns(vec![sheet.default_column; sheet.num_columns])
                .num_rows(sheet.num_rows)
                .num_sticky_cols(1);

            table.show(ui, sheet);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_letter() {
        let res = Sheet::idx_to_column_letter(702);

        assert_eq!(res, "AAA");
    }
}
