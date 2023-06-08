use std::{io::Write, thread::sleep, time::Duration};

use anyhow::*;
use serialport::SerialPort;

pub const COMMAND_RIGHT_HALFSTEP: &'static str = "$J=G21G91X0.3Y0.3F50";
pub const COMMAND_LEFT_HALFSTEP: &'static str = "$J=G21G91X-0.3Y-0.3F50";
pub const COMMAND_UP_HALFSTEP: &'static str = "$J=G21G91X0.3Y-0.3F50";
pub const COMMAND_DOWN_HALFSTEP: &'static str = "$J=G21G91X-0.3Y0.3F50";
pub const COMMAND_EMAG_ON: &'static str = "E1";
pub const COMMAND_EMAG_OFF: &'static str = "E0";

static mut SERIALPORT: Option<Box<dyn SerialPort>> = None;
static mut CURRENT: (f64, f64) = (0.0, 0.0);

pub fn emag_on() -> Result<()> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "{COMMAND_EMAG_ON}")?;

    Ok(())
}

pub fn emag_off() -> Result<()> {
    writeln!(
        unsafe { SERIALPORT.as_mut().unwrap() },
        "{COMMAND_EMAG_OFF}"
    )?;

    Ok(())
}

pub fn up_half() -> Result<()> {
    if unsafe { CURRENT.1 } >= 3.0 {
        bail!("Cannot go up any further");
    }

    writeln!(
        unsafe { SERIALPORT.as_mut().unwrap() },
        "{COMMAND_UP_HALFSTEP}"
    )?;

    unsafe { CURRENT = (CURRENT.0, CURRENT.1 + 0.5) }

    block_until_idle();

    Ok(())
}

pub fn up_full() -> Result<()> {
    up_half()?;
    up_half()?;

    Ok(())
}

pub fn down_half() -> Result<()> {
    if unsafe { CURRENT.1 } <= 0.0 {
        bail!("Cannot go down any further");
    }

    writeln!(
        unsafe { SERIALPORT.as_mut().unwrap() },
        "{COMMAND_DOWN_HALFSTEP}"
    )?;

    unsafe { CURRENT = (CURRENT.0, CURRENT.1 - 0.5) }

    block_until_idle();

    Ok(())
}

pub fn down_full() -> Result<()> {
    down_half()?;
    down_half()?;

    Ok(())
}

pub fn left_half() -> Result<()> {
    if unsafe { CURRENT.0 } <= 0.0 {
        bail!("Cannot go left any further");
    }

    writeln!(
        unsafe { SERIALPORT.as_mut().unwrap() },
        "{COMMAND_LEFT_HALFSTEP}"
    )?;

    unsafe { CURRENT = (CURRENT.0 - 0.5, CURRENT.1) }

    block_until_idle();

    Ok(())
}

pub fn left_full() -> Result<()> {
    left_half()?;
    left_half()?;

    Ok(())
}

pub fn right_half() -> Result<()> {
    if unsafe { CURRENT.0 } >= 3.0 {
        bail!("Cannot go right any further");
    }

    writeln!(
        unsafe { SERIALPORT.as_mut().unwrap() },
        "{COMMAND_RIGHT_HALFSTEP}"
    )?;

    unsafe { CURRENT = (CURRENT.0 + 0.5, CURRENT.1) }

    block_until_idle();

    Ok(())
}

pub fn right_full() -> Result<()> {
    right_half()?;
    right_half()?;

    Ok(())
}

pub fn origin() -> Result<()> {
    goto((0.0, 0.0))?;

    Ok(())
}

pub fn goto(pos: (f64, f64)) -> Result<()> {
    let (x_to, y_to) = pos.clone();

    // x_to and y_to must be a .5 or .0
    if x_to.fract() != 0.0 && x_to.fract() != 0.5 {
        bail!("x_to must be a .5 or .0");
    }

    if y_to.fract() != 0.0 && y_to.fract() != 0.5 {
        bail!("y_to must be a .5 or .0");
    }

    // x_to and y_to must be between 0 and 3
    if x_to < 0.0 || x_to > 3.0 {
        bail!("x_to must be between 0 and 3");
    }

    if y_to < 0.0 || y_to > 3.0 {
        bail!("y_to must be between 0 and 3");
    }

    let going_up = y_to > unsafe { get_current_pos() }.1;
    let going_right = x_to > unsafe { get_current_pos() }.0;

    while unsafe { get_current_pos() }.1 != y_to {
        if going_up {
            up_half()?;
        } else {
            down_half()?;
        }
    }

    while unsafe { get_current_pos() }.0 != x_to {
        if going_right {
            right_half()?;
        } else {
            left_half()?;
        }
    }

    Ok(())
}

pub unsafe fn get_current_pos() -> (f64, f64) {
    CURRENT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Idle,
    Working,
}

pub fn status() -> Result<Status> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "$?")?;

    // read to string until the | character
    let mut buf = String::new();

    loop {
        let mut byte = [0; 1];
        unsafe { SERIALPORT.as_mut().unwrap() }.read_exact(&mut byte)?;

        if byte[0] == '|' as u8 {
            break;
        }

        buf.push(byte[0] as char);
    }

    // read until newline
    loop {
        let mut byte = [0; 1];
        unsafe { SERIALPORT.as_mut().unwrap() }.read_exact(&mut byte)?;

        if byte[0] == '\n' as u8 {
            break;
        }
    }

    if buf.to_lowercase().contains("idle") {
        Ok(Status::Idle)
    } else {
        Ok(Status::Working)
    }
}

fn block_until_idle() {
    let mut iterations = 0;

    loop {
        if status().unwrap() == Status::Idle {
            break;
        }

        if iterations > 10000 {
            panic!("timed out waiting for idle");
        }

        iterations += 1;
        sleep(Duration::from_millis(100));
    }
}

pub fn init(s: Box<dyn SerialPort>) {
    unsafe {
        SERIALPORT = Some(s);
    }
}

pub fn disconnect() -> Result<()> {
    unsafe {
        let p = match SERIALPORT.take() {
            Some(s) => s,
            None => return Ok(()),
        };

        drop(p);
    }

    Ok(())
}

pub fn connected() -> bool {
    unsafe { SERIALPORT.is_some() }
}
