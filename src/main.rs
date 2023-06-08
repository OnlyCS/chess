#![warn(clippy::unwrap_used)]
#![allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
#![feature(drain_filter, file_create_new, let_chains)]

extern crate anyhow;
extern crate clap;
extern crate intuitive;
extern crate promptly;
extern crate rand;

pub mod cli;
pub mod core;
pub mod grbl;
pub mod ui;
pub mod utils;

use std::{process::exit, time::Duration};

use anyhow::*;
use clap::Parser;
use intuitive::terminal::Terminal;
use promptly::prompt;
use serialport::SerialPortType;

use crate::{
    cli::args::Cli, core::hexapawn::HexapawnBoard, ui::hex_ui::Root as HexRoot,
    ui::root::Root as ChessRoot,
};

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.manual {
        if args.hexapawn {
            bail!("Manual mode incompatible with game arguments. Omit \"-x\"");
        }

        if !args.serial {
            bail!("Manual mode needs serial to communicate with grbl. Use \"-s\"");
        }

        {
            let ports = serialport::available_ports().expect("No ports found");
            let mut port_name = Default::default();

            for p in ports {
                if let SerialPortType::UsbPort(usb) = p.port_type {
                    if let Some(name) = usb.product {
                        if name.to_lowercase().contains("uno") {
                            port_name = p.port_name;
                            break;
                        }
                    }
                }
            }

            if port_name == String::default() {
                panic!("Failed to find Arduino Uno");
            }

            let port = serialport::new(port_name, 115200)
                .timeout(Duration::from_secs(5))
                .open()
                .expect("Failed to open Arduino Uno");

            grbl::init(port);
        }

        println!("please remember to set head to (0,0)");

        loop {
            println!("1:up\n2:down\n3:left\n4:right\n5:emag on\n6:emag off\n7:quit");
            let opt: u8 = prompt("Enter option").unwrap();

            let r = match opt {
                1 => grbl::up_half(),
                2 => grbl::down_half(),
                3 => grbl::left_half(),
                4 => grbl::right_half(),
                5 => grbl::emag_on(),
                6 => grbl::emag_off(),
                7 => exit(0),
                _ => Err(anyhow!("Invalid option")),
            };

            match r {
                std::result::Result::Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    if args.hexapawn {
        Terminal::new(HexRoot::with_board(HexapawnBoard::new(args.serial)))?.run()?;
    } else if args.serial {
        bail!("serial is only supported for hexapawn. Try using \"-x\"")
    } else {
        Terminal::new(ChessRoot::new())?.run()?;
    }

    Ok(())
}
