use intuitive::{
    components::*,
    event::{KeyHandler, MouseHandler},
    *,
};

use super::{data::UIFileData, file::File};

#[component(Board)]
pub fn render(on_key: KeyHandler, on_mouse: MouseHandler, board_data: Vec<UIFileData>) {
    render! {
            HStack(on_key: on_key, on_mouse: on_mouse) {
                File(data: board_data[0].clone().pieces, selected: board_data[0].clone().select)
                File(data: board_data[1].clone().pieces, selected: board_data[1].clone().select)
                File(data: board_data[2].clone().pieces, selected: board_data[2].clone().select)
                File(data: board_data[3].clone().pieces, selected: board_data[3].clone().select)
                File(data: board_data[4].clone().pieces, selected: board_data[4].clone().select)
                File(data: board_data[5].clone().pieces, selected: board_data[5].clone().select)
                File(data: board_data[6].clone().pieces, selected: board_data[6].clone().select)
                File(data: board_data[7].clone().pieces, selected: board_data[7].clone().select)
            }
    }
}
