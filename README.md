# chess-ai

This was going to be a coding adventure into making neural networks and stuff but I decided not
to due to lack of intrest (I want to move on to other things). This was still a fun adventure
into the world of Chess, Rust, and really, *really* bad tui libraries.

This will probably be the basis for my electronic chess board project.

## roadmap

- [x] implement all pieces
- [x] implement move generation
- [x] full tui
- [] maybe networking/server

## stockfish server setup

on linux:
```bash
socat tcp-listen:1234,reuseaddr exec:/path/to/stockfish
```

on windows (gotta install winsocat/idk if this works, chatgpt said so):
```powershell
winsocat tcp-listen:1234,reuseaddr exec:/path/to/stockfish
```