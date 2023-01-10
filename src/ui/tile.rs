use intuitive::{components::*, *};

#[component(Tile)]
pub fn render(piece_str: String) {
    let mut text_str = " ".to_string();
    text_str.push_str(piece_str);

    render! {
        Section() {
            HStack(flex: [5, 12, 4]) {
                Empty()
                Text(text: text_str)
                Empty()
            }
        }
    }
}
