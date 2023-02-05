use intuitive::{
    components::*,
    style::{Color, Modifier, Style},
    *,
};

use super::selection::SelectionType;

#[component(Square)]
pub fn render(piece: String, selected: Option<SelectionType>) {
    let style = match selected {
        Some(SelectionType::Hover) => Style::new(Some(Color::Blue), None, Modifier::empty()),
        Some(SelectionType::Selected) => Style::new(Some(Color::Green), None, Modifier::empty()),
        Some(SelectionType::Available) => Style::new(Some(Color::Yellow), None, Modifier::empty()),
        None => Style::new(None, None, Modifier::empty()),
    };

    render! {
        Section(border: style) {
            HStack(flex: [5, 12, 4]) {
                Empty()
                Text(text: piece)
                Empty()
            }
        }
    }
}
