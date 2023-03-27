use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

use anyhow::{bail, Context, Result};

pub fn eval(fen: String) -> Result<String> {
    let mut stream = TcpStream::connect("localhost:1234")?;

    stream.write_all(format!("position fen {}\n", fen).as_bytes())?;
    stream.write_all("eval\n".as_bytes())?;

    let mut reader = BufReader::new(stream.try_clone()?);

    let mut buffer = String::new();

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

        if buffer.ends_with("with scaled NNUE") {
            break;
        }
    }

    Ok(buffer)
}
