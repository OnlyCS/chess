pub fn get_instructions(size: (u16, u16)) -> String {
    let too_small = size.0 < 40 || size.1 < 26;

    if too_small {
        format!(
            "Increase terminal size\nCurrent: {}x{}\nNeeds to be 50x26",
            size.0, size.1
        )
    } else {
        format!(
            "← ↑ ↓ →: Move selection\nEnter: Select\nq: Quit\n{} {}",
            size.0, size.1
        )
    }
}

pub fn board_flex(size: (u16, u16)) -> (u16, u16, u16, u16) {
    let w = size.0;
    let h = size.1;

    let mut board_w = 50;
    let mut board_h = 26;

    while board_w + 50 <= w && board_h + 26 <= h {
        board_w += 50;
        board_h += 26;
    }

    (board_w, w - board_w, board_h, h - board_h) as (u16, u16, u16, u16)
}
