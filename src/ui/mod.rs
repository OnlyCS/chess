pub mod data;
pub mod file;
pub mod root;
pub mod selection;
pub mod square;

/*
pub fn board_flex(size: (u16, u16)) -> (u16, u16, u16, u16) {
    let w = size.0;
    let h = size.1;

    let board_w = 50;
    let board_h = 26;

    // while board_w + 50 <= w && board_h + 26 <= h {
    //     board_w += 50;
    //     board_h += 26;
    // }

    if w >= board_w && h >= board_h && w >= 91 && h >= 27 {
        (board_w, w - board_w, board_h, h - board_h) as (u16, u16, u16, u16)
    } else {
        (0, 1, 1, 0)
    }
}


91x27!!
*/
