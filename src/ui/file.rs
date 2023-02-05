use super::{selection::SelectionType, square::Square};
use intuitive::{components::VStack, *};

#[component(File)]
pub fn render(data: Vec<String>, selected: Vec<Option<SelectionType>>) {
    render! {
        VStack(flex: [1, 1, 1, 1, 1, 1, 1, 1]) {
            Square(piece: data[0].clone(), selected: selected[0].clone())
            Square(piece: data[1].clone(), selected: selected[1].clone())
            Square(piece: data[2].clone(), selected: selected[2].clone())
            Square(piece: data[3].clone(), selected: selected[3].clone())
            Square(piece: data[4].clone(), selected: selected[4].clone())
            Square(piece: data[5].clone(), selected: selected[5].clone())
            Square(piece: data[6].clone(), selected: selected[6].clone())
            Square(piece: data[7].clone(), selected: selected[7].clone())
        }
    }
}
