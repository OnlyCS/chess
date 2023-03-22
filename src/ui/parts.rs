use intuitive::{
    components::{experimental::modal::use_modal, *},
    event::{KeyHandler, MouseHandler},
    style::{Color, Modifier, Style},
    *,
};

use crate::{
    core::{file::File, square::Square},
    ui::selection::Selection,
};

#[component(SquareComponent)]
pub fn render(square: Square, selection: Selection) {
    let style = match selection {
        Selection::SelectPiece(hovered_pc) => {
            if square.get_position() == hovered_pc {
                Style::new(Some(Color::Green), None, Modifier::empty())
            } else {
                Style::new(None, None, Modifier::empty())
            }
        }
        Selection::SelectMove(hovered_sq, selected_pc, available_moves) => {
            if square.get_position() == hovered_sq {
                Style::new(Some(Color::Blue), None, Modifier::empty())
            } else if square.get_position() == selected_pc {
                Style::new(Some(Color::Green), None, Modifier::empty())
            } else if available_moves.contains(square.get_position()) {
                Style::new(Some(Color::Yellow), None, Modifier::empty())
            } else {
                Style::new(None, None, Modifier::empty())
            }
        }
    };

    render! {
        Section(border: style) {
            HStack(flex: [2, 3, 2]) {
                Empty()
                Text(text: square.get_piece().map(|p| p.to_string()).unwrap_or_else(|| " ".to_string()))
                Empty()
            }
        }
    }
}

#[component(FileComponent)]
pub fn render(file: File, selection: Selection) {
    render! {
        VStack() {
            SquareComponent(square: file.squares[0].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[1].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[2].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[3].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[4].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[5].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[6].clone(), selection: selection.clone())
            SquareComponent(square: file.squares[7].clone(), selection: selection.clone())
        }
    }
}

#[component(BoardComponent)]
pub fn board(on_key: KeyHandler, on_mouse: MouseHandler, board: Vec<File>, selection: Selection) {
    render! {
        HStack(on_key: on_key, on_mouse: on_mouse) {
            FileComponent(file: board[0].clone(), selection: selection.clone())
            FileComponent(file: board[1].clone(), selection: selection.clone())
            FileComponent(file: board[2].clone(), selection: selection.clone())
            FileComponent(file: board[3].clone(), selection: selection.clone())
            FileComponent(file: board[4].clone(), selection: selection.clone())
            FileComponent(file: board[5].clone(), selection: selection.clone())
            FileComponent(file: board[6].clone(), selection: selection.clone())
            FileComponent(file: board[7].clone(), selection: selection.clone())
        }
    }
}

#[component(PromoteSection)]
pub fn render(promote: bool) {
    let modal = use_modal();

    if *promote && !modal.is_shown() {
        modal.show(render! {
			Section(title: "Promote") {
				Text(text: "You are now eligible to promote!\n Choose one of the following: \n(B)ishop\n(K)night\n(Q)ueen\n(R)ook")
			}
		})
    } else if !*promote && modal.is_shown() {
        modal.hide();
    }

    render! {
        Section(title: "Promote") {
            Text(text: "No promotion available")
        }
    }
}
