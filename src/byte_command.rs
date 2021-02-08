use crate::{Command, Key, MouseButton};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CommandCode {
    KeyDown,
    KeyUp,
    KeyClick,
    MouseMoveRel,
    MouseMoveAbs,
    MouseWarp,
    MouseScroll,
    MouseDown,
    MouseUp,
    MouseClick,
}

impl CommandCode {
    pub const COUNT: u8 = Self::MouseClick as u8 + 1;
}

pub enum ParseByteCommandError {
    InvalidCommandCode(u8),
    InvalidKey(u8),
    InvalidMouseButton(u8),
    BufferTooShort(usize),
}

use ParseByteCommandError::*;

fn parse_int(byte_0: u8, byte_1: u8) -> i32 {
    (((byte_0 as i16) << 8) | (byte_1 as i16)) as i32
}

fn parse_command_code(byte: u8) -> Result<CommandCode, ParseByteCommandError> {
    if byte < CommandCode::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidCommandCode(byte))
    }
}

fn parse_key(byte: u8) -> Result<Key, ParseByteCommandError> {
    if byte < Key::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidKey(byte))
    }
}

fn parse_mouse_button(byte: u8) -> Result<MouseButton, ParseByteCommandError> {
    if byte < MouseButton::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidMouseButton(byte))
    }
}

fn check_buffer_length(buf: &[u8], len: usize) -> Result<(), ParseByteCommandError> {
    if buf.len() >= len {
        Ok(())
    } else {
        // Should we put expected, actual, or both?
        Err(BufferTooShort(buf.len()))
    }
}

pub fn parse_byte_command(buf: &[u8]) -> Result<(Command, usize), ParseByteCommandError> {
    if buf.len() == 0 {
        return Err(BufferTooShort(0));
    }

    match parse_command_code(buf[0])? {
        CommandCode::KeyDown => {
            check_buffer_length(buf, 3)?;
            Ok((Command::KeyDown(parse_key(buf[1])?, buf[2] != 0), 3))
        },
        CommandCode::KeyUp => {
            check_buffer_length(buf, 2)?;
            Ok((Command::KeyUp(parse_key(buf[1])?), 2))
        },
        CommandCode::KeyClick => {
            check_buffer_length(buf, 2)?;
            Ok((Command::KeyClick(parse_key(buf[1])?), 2))
        },
        CommandCode::MouseMoveRel => {
            check_buffer_length(buf, 5)?;
            Ok((Command::MouseMoveRel(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
        },
        CommandCode::MouseMoveAbs => {
            check_buffer_length(buf, 5)?;
            Ok((Command::MouseMoveAbs(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
        },
        CommandCode::MouseWarp => {
            check_buffer_length(buf, 5)?;
            Ok((Command::MouseWarp(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
        },
        CommandCode::MouseScroll => {
            check_buffer_length(buf, 5)?;
            Ok((Command::MouseScroll(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
        },
        CommandCode::MouseDown => {
            check_buffer_length(buf, 3)?;
            Ok((Command::MouseDown(parse_mouse_button(buf[1])?, buf[2] as u32), 3))
        },
        CommandCode::MouseUp => {
            check_buffer_length(buf, 3)?;
            Ok((Command::MouseUp(parse_mouse_button(buf[1])?, buf[2] as u32), 3))
        },
        CommandCode::MouseClick => {
            check_buffer_length(buf, 3)?;
            Ok((Command::MouseClick(parse_mouse_button(buf[1])?, buf[2] as u32), 3))
        },
    }
}