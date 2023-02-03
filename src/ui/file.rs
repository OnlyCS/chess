use super::square::Square;
use intuitive::{components::VStack, *};

#[component(File)]
pub fn render(data: [String; 8]) {
    render! {
        VStack(flex: [1, 1, 1, 1, 1, 1, 1, 1]) {
            Square(piece: data[0].clone())
            Square(piece: data[1].clone())
            Square(piece: data[2].clone())
            Square(piece: data[3].clone())
            Square(piece: data[4].clone())
            Square(piece: data[5].clone())
            Square(piece: data[6].clone())
            Square(piece: data[7].clone())
        }
    }
}
