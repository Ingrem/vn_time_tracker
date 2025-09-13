use eframe::egui::{Button, Color32, Direction, FontId, Layout, RichText, TextEdit, Ui, Vec2};
use egui_extras::TableRow;

/// Common button styles mapped to colors
pub enum ButtonStyle {
    Success, // positive action
    Neutral, // secondary action
    Danger,  // destructive action
}

impl ButtonStyle {
    pub fn color(&self) -> Color32 {
        match self {
            ButtonStyle::Success => Color32::from_rgb(80, 120, 80),
            ButtonStyle::Neutral => Color32::from_rgb(96, 125, 139),
            ButtonStyle::Danger => Color32::from_rgb(160, 80, 80),
        }
    }
}

/// Draw a styled button with given label and size.
pub fn action_button(ui: &mut Ui, text: &str, size: Vec2, style: Option<ButtonStyle>) -> eframe::egui::Response {
    let mut button = Button::new(RichText::new(text).size(16.0)).min_size(size);

    if let Some(style) = style {
        button = button.fill(style.color());
    }

    ui.add(button)
}

/// Draw a centered button inside a table cell.
pub fn centered_button(ui: &mut Ui, text: &str, size: Vec2, style: Option<ButtonStyle>) -> bool {
    let mut clicked = false;
    ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
        if action_button(ui, text, size, style).clicked() {
            clicked = true;
        }
    });
    clicked
}

/// Draw a centered header cell.
pub fn header_cell(row: &mut TableRow<'_, '_>, text: &str) {
    row.col(|ui| {
        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            ui.label(RichText::new(text).size(20.0).strong());
        });
    });
}

/// Draw two centered buttons with given spacing.
/// Returns a tuple of (first_clicked, second_clicked).
pub fn centered_two_buttons(
    ui: &mut Ui,
    labels: (&str, &str),
    size: Vec2,
    spacing: f32,
    offset: f32,
    styles: (Option<ButtonStyle>, Option<ButtonStyle>),
) -> (bool, bool) {
    let total_width = size.x * 2.0 + spacing;
    let margin = (ui.available_width() - total_width) / 2.0 - offset;

    ui.horizontal(|ui| {
        ui.add_space(margin.max(0.0));

        let first = action_button(ui, labels.0, size, styles.0).clicked();

        ui.add_space(spacing);

        let second = action_button(ui, labels.1, size, styles.1).clicked();

        (first, second)
    })
    .inner
}

/// Draws a labeled single-line text field.
pub fn labeled_text_edit(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    label_size: f32,
    edit_size: Vec2,
    font_size: f32,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(label_size));
        ui.add_space(10.0);
        let edit = TextEdit::singleline(value)
            .font(FontId::proportional(font_size))
            .background_color(Color32::from_rgb(200, 200, 200))
            .text_color(Color32::from_rgb(40, 40, 40));
        ui.add_sized(edit_size, edit);
    });
}
