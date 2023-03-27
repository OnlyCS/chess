use anyhow::{bail, Context, Result};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    process::{self, Command},
};
use zip::ZipArchive;

const STOCKFISH_URL_WIN: &str = "https://stockfishchess.org/files/stockfish_15.1_win_x64_avx2.zip";
const STOCKFISH_URL_LINUX: &str =
    "https://stockfishchess.org/files/stockfish_15.1_linux_x64_avx2.zip";

const STOCKFISH_DIR_WIN: &str =
    "stockfish/stockfish_15.1_win_x64_avx2/stockfish-windows-2022-x86-64-avx2.exe";
const STOCKFISH_DIR_LINUX: &str =
    "stockfish/stockfish_15.1_linux_x64_avx2/stockfish-ubuntu-20.04-x86-64-avx2";

pub fn download() -> Result<()> {
    let url = if cfg!(target_os = "windows") {
        STOCKFISH_URL_WIN
    } else {
        STOCKFISH_URL_LINUX
    };

    let mut resp = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create("stockfish.zip")?;
    std::io::copy(&mut resp, &mut file)?;

    Ok(())
}

pub fn extract() -> Result<()> {
    let mut archive = ZipArchive::new(fs::File::open("stockfish.zip")?)?;
    archive.extract("stockfish")?;

    Ok(())
}

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
