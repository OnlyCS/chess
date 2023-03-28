use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

use anyhow::{bail, Context, Result};

fn readall(buffer: &mut String, reader: &mut BufReader<TcpStream>, endon: &str) -> Result<()> {
    loop {
        let byte = reader.get_mut().take(1).bytes().next();
        match byte {
            Some(Ok(b)) => {
                buffer.push(b as char);
            }
            Some(Err(e)) => {
                Result::Err(e).context("Failed to read from stream")?;
            }
            None => {
                bail!("Failed to read from stream");
            }
        }

        if buffer.ends_with(endon) {
            break;
        }
    }

    Ok(())
}

pub fn eval(fen: String) -> Result<f64> {
    let mut stream = TcpStream::connect("localhost:1234")?;

    stream.write_all(format!("position fen {}\n", fen).as_bytes())?;

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut buffer = String::new();

    stream.write_all("go depth 10\n".as_bytes())?;
    readall(&mut buffer, &mut reader, "bestmove")?;

    // get 2nd to last line of buffer
    let mut lines = buffer.lines().collect::<Vec<_>>();
    lines.pop();
    let last = lines.last().context("Could not get last line")?;

    if last.contains("mate") {
        let matein = last
            .split("mate")
            .nth(1)
            .context("failed")?
            .split(' ')
            .nth(1)
            .context("failed")?
            .parse::<i32>()?;

        return Ok((1000 * (10 - matein)) as f64);
    }

    buffer.clear();

    stream.write_all(b"eval\n")?;
    readall(&mut buffer, &mut reader, "with scaled NNUE")?;

    // get last line
    let lines = buffer.lines();
    let last_line = lines.last().context("Could not get last line fsr")?;

    // last line looks like this:
    // Final evaluation       +4.68 (white side) [with scaled NNUE
    // resolve to: 4.68
    // this CAN BE NEGATIVE

    let mut split = last_line.split_whitespace();
    let score = split
        .nth(2)
        .context("Could not get score... did the Stockfish API Change?")?;

    // score is +x.xx or -x.xx. resolve to f64
    let score = score.parse::<f64>()?;

    Ok(score)
}
