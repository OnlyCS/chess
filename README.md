# chess-ai

Now, in rust. What a beauty.

`src/ui` contains everything used in the terminal-user-interface

`src/game` contains a wrapper for the board and turn management

`src/parts` contains the board, and data wrappers for squares, files, and coordinates

`src/types` is a synonym for the utils folder. It contains enums for the color, a `move` struct with a `from` and `to` position, ...etc.

`src/pieces` contains implementations for all of the pieces, including move generation.

## roadmap

- [x] implement all pieces
- [x] implement move generation
- [x] full tui
- [] conversion to uci and other standard chess protocols
- [] the ai part (will train on pleco)
- [] maybe networking in the future

## remember you need a server running stockfish (like tcpip)

on linux:
```bash
socat tcp-listen:8888,reuseaddr exec:/path/to/stockfish
```