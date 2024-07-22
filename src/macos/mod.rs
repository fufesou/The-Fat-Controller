mod error;
mod ffi;
mod keyboard;
mod mouse;
mod screen;

use error::PlatformError;
type Error = crate::GenericError<PlatformError>;
use core_graphics::event_source::CGEventSource;

const SHIFT_BIT: u32 = 0;
const OPTION_BIT: u32 = 1;

#[derive(Copy, Clone)]
struct KeyInfo {
    key_code: u8,
    modifiers: u8,
}

/// The main context used for generating events (macOS).
///
/// The most useful methods are on the [`traits`](crate::traits).
pub struct Context {
    hid_connect: ffi::io_connect_t,
    event_source: Box<MyCGEventSource>,
    modifiers: ffi::IOOptionBits,
    button_state: u8,
    key_map: std::collections::HashMap<char, KeyInfo>,
}

#[derive(Clone)]
pub struct MyCGEventSource(CGEventSource);
unsafe impl Sync for MyCGEventSource {}
unsafe impl Send for MyCGEventSource {}

fn connect_to_service(name: *const u8, connect_type: u32) -> Result<ffi::io_connect_t, Error> {
    unsafe {
        // Create a dictionary that describes a service matching a name.
        let matching = ffi::IOServiceMatching(name);

        // Get an iterator to all IOService objects that match the
        // dictionary. IOServiceGetMatchingServices will release the
        // dictionary.
        let mut iterator = ffi::IO_OBJECT_NULL;
        let error_code =
            ffi::IOServiceGetMatchingServices(ffi::kIOMasterPortDefault, matching, &mut iterator);
        if error_code != ffi::kIOReturnSuccess {
            return Err(Error::Platform(PlatformError::new(error_code)));
        }

        let mut found = false;
        let mut service;
        let mut connect = ffi::IO_OBJECT_NULL;

        // Consume the iterator and check each IOService object.
        loop {
            service = ffi::IOIteratorNext(iterator);
            if service == ffi::IO_OBJECT_NULL {
                break;
            }

            // Try to open a connection to the IOService. If successful,
            // we're done. We don't need a reference to the service after
            // opening a connection to it.
            if ffi::IOServiceOpen(service, ffi::mach_task_self_, connect_type, &mut connect)
                == ffi::kIOReturnSuccess
            {
                found = true;
                ffi::IOObjectRelease(service);
                break;
            }

            ffi::IOObjectRelease(service);
        }

        ffi::IOObjectRelease(iterator);

        if found {
            Ok(connect)
        } else {
            Err(Error::Unknown)
        }
    }
}

fn create_key_map() -> Result<std::collections::HashMap<char, KeyInfo>, Error> {
    // Iterate over all combinations of key codes and modifier states and
    // convert them to characters. Use this to create a mapping from characters
    // to key codes and modifier states.
    use std::collections::hash_map::{Entry, HashMap};
    use std::ffi::c_void;

    const MAX_STRING_LENGTH: usize = 255;

    let input_source;
    let layout;
    unsafe {
        input_source = ffi::TISCopyCurrentKeyboardLayoutInputSource();
        if input_source.is_null() {
            return Err(Error::Unknown);
        }
        let layout_data =
            ffi::TISGetInputSourceProperty(input_source, ffi::kTISPropertyUnicodeKeyLayoutData);
        if layout_data.is_null() {
            ffi::CFRelease(input_source as *mut c_void);
            return Err(Error::Unknown);
        }
        layout = ffi::CFDataGetBytePtr(layout_data) as *const ffi::UCKeyboardLayout;
    }
    let keyboard_type = unsafe { ffi::LMGetKbdType() };

    let mut key_map = HashMap::new();

    let mut dead_keys = 0;
    let mut length = 0;
    let mut string: [ffi::UniChar; MAX_STRING_LENGTH] = [0; MAX_STRING_LENGTH];

    for mod_idx in 0..4 {
        // The modifier flags that UCKeyTranslate takes are the ones defined
        // in events.h shifted right by 8. So shift is shiftKeyBit >> 8.
        let shift_bit = (mod_idx & (1 << SHIFT_BIT)) << (ffi::shiftKeyBit - SHIFT_BIT - 8);
        let option_bit = (mod_idx & (1 << OPTION_BIT)) << (ffi::optionKeyBit - OPTION_BIT - 8);
        let modifiers = shift_bit | option_bit;

        for key_code in 0..128 {
            // UCKeyTranslate takes a key code, the state of the modifiers,
            // and a keyboard layout to produce the UTF-16 string that would
            // be typed if the key was pressed with the modifiers.
            let status = unsafe {
                ffi::UCKeyTranslate(
                    layout,
                    key_code,
                    ffi::kUCKeyActionDisplay,
                    modifiers,
                    keyboard_type as u32,
                    ffi::kUCKeyTranslateNoDeadKeysMask,
                    &mut dead_keys,
                    MAX_STRING_LENGTH as ffi::UniCharCount,
                    &mut length,
                    string.as_mut_ptr(),
                )
            };

            if status != 0 {
                unsafe {
                    ffi::CFRelease(input_source as *mut c_void);
                }
                return Err(Error::Unknown);
            }

            let string_utf16 = &string[..length as usize];
            let string_utf32 =
                std::char::decode_utf16(string_utf16.iter().cloned()).collect::<Vec<_>>();

            if string_utf32.len() == 1 {
                if let Ok(ch) = string_utf32[0] {
                    if let Entry::Vacant(entry) = key_map.entry(ch) {
                        entry.insert(KeyInfo {
                            key_code: key_code as u8,
                            modifiers: mod_idx as u8,
                        });
                    }
                }
            }
        }
    }

    unsafe {
        ffi::CFRelease(input_source as *mut c_void);
    }

    // UCKeyTranslate seems to produce a carriage-return instead of a
    // line-feed when given kVK_Return. That's kinda weird. Maybe it's
    // because macOS used carriage-return as the newline character in the
    // ancient times?
    key_map.insert(
        '\n',
        KeyInfo {
            key_code: ffi::kVK_Return,
            modifiers: 0,
        },
    );

    Ok(key_map)
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let key_map = create_key_map()?;

        let event_source = match CGEventSource::new(CGEventSourceStateID::Private) {
            Ok(s) => s,
            Err(()) => return Err(Error::Unknown),
        };

        let hid_connect =
            connect_to_service(ffi::kIOHIDSystemClass.as_ptr(), ffi::kIOHIDParamConnectType)?;

        Ok(Self {
            hid_connect,
            event_source: Box::new(MyCGEventSource(event_source)),
            modifiers: 0,
            button_state: 0,
            key_map,
        })
    }

    fn post_event(
        &self,
        event_type: u32,
        event: *const ffi::NXEventData,
        flags: ffi::IOOptionBits,
        options: ffi::IOOptionBits,
    ) -> Result<(), Error> {
        let error_code = unsafe {
            ffi::IOHIDPostEvent(
                self.hid_connect,
                event_type,
                ffi::IOGPoint { x: 0, y: 0 },
                event,
                ffi::kNXEventDataVersion,
                flags,
                options,
            )
        };
        if error_code == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(Error::Platform(PlatformError::new(error_code)))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::IOServiceClose(self.hid_connect);
        }
    }
}
