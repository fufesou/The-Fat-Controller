// https://github.com/torvalds/linux/blob/master/include/uapi/linux/input.h

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct timeval {
    pub tv_sec: isize,
    pub tv_usec: isize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct input_event {
    pub time: timeval,
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct input_id {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

pub const BUS_USB: u16 = 0x03;
