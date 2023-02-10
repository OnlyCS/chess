# chess-ai

Now, in rust. What a beauty.

`src/ui` contains everything used in the terminal-user-interface

`src/game` contains a wrapper for the board and turn management

`src/parts` contains the board, and data wrappers for squares, files, and coordinates

`src/types` is a synonym for the utils folder. It contains enums for the color, a `move` struct with a `from` and `to` position, ...etc.

`src/pieces` contains implementations for all of the pieces, including move generation.

This project depends on the [intuitive](https://docs.rs/intuitive/latest/intuitive/index.html) TUI library to make terminal look pretty, because that's
what really matters at the end of the day. This project will also have an AI using NEAT 
(probably using [this](https://github.com/TLmaK0/rustneat) library), trained on stockfish.
