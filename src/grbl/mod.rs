use anyhow::*;
use serialport::SerialPort;

pub const RIGHT_F100: &'static str = "$J=G21G91X1Y1F100";
pub const LEFT_F100: &'static str = "$J=G21G91X-1Y-1F100";
pub const UP_F100: &'static str = "$J=G21G91X1Y-1F100";
pub const DOWN_F100: &'static str = "$J=G21G91X-1Y1F100";

static mut SERIALPORT: Option<Box<dyn SerialPort>> = None;
static mut CURRENT: (f64, f64) = (0.0, 0.0);

pub fn up_half() -> Result<()> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "{UP_F100}")?;

    unsafe { CURRENT.1 += 0.5 }

    Ok(())
}

pub fn up_full() -> Result<()> {
    up_half()?;
    up_half()?;

    Ok(())
}

pub fn down_half() -> Result<()> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "{DOWN_F100}")?;

    unsafe { CURRENT.1 -= 0.5 }

    Ok(())
}

pub fn down_full() -> Result<()> {
    down_half()?;
    down_half()?;

    Ok(())
}

pub fn left_half() -> Result<()> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "{LEFT_F100}")?;

    unsafe { CURRENT.0 -= 0.5 }

    Ok(())
}

pub fn left_full() -> Result<()> {
    left_half()?;
    left_half()?;

    Ok(())
}

pub fn right_half() -> Result<()> {
    writeln!(unsafe { SERIALPORT.as_mut().unwrap() }, "{RIGHT_F100}")?;

    unsafe { CURRENT.0 += 0.5 }

    Ok(())
}

pub fn right_full() -> Result<()> {
    right_half()?;
    right_half()?;

    Ok(())
}

pub fn origin() -> Result<()> {
    goto((0.0, 0.0))
}

pub fn goto(pos: (f64, f64)) -> Result<()> {
    let (x_to, y_to) = pos.clone();
    let (x_from, y_from) = unsafe { CURRENT };

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

    let going_up = y_to > y_from;
    let going_right = x_to > x_from;

    let mut y = y_from;
    let mut x = x_from;

    while y != y_to {
        if going_up {
            up_half()?;
            y += 0.5;
        } else {
            down_half()?;
            y -= 0.5;
        }
    }

    while x != x_to {
        if going_right {
            right_half()?;
            x += 0.5;
        } else {
            left_half()?;
            x -= 0.5;
        }
    }

    unsafe { CURRENT = (x_to, y_to) };

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
