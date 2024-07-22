use super::ffi;
use crate::{utils::NonZero, FallibleContext};
use std::fmt::{self, Display, Formatter};

type NonZeroDWORD = <ffi::DWORD as NonZero>::Type;

#[derive(Debug)]
pub struct PlatformError(NonZeroDWORD);

impl PlatformError {
    pub(super) fn last() -> Self {
        unsafe { Self(NonZeroDWORD::new_unchecked(ffi::GetLastError())) }
    }
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let message_buffer: ffi::LPCWSTR = std::ptr::null();

            let message_length = ffi::FormatMessageW(
                ffi::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | ffi::FORMAT_MESSAGE_FROM_SYSTEM
                    | ffi::FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null(),
                self.0.get(),
                0,
                std::ptr::addr_of!(message_buffer) as *mut ffi::WCHAR,
                0,
                std::ptr::null_mut(),
            );

            if message_length == 0 {
                return write!(f, "Error code: {}", self.0.get());
            }

            // Removing CRLF and period.
            let message_length = (message_length - 3) as usize;
            let message = std::slice::from_raw_parts(message_buffer, message_length);
            let result = write!(f, "{}", String::from_utf16_lossy(message));

            ffi::LocalFree(message_buffer as *mut std::ffi::c_void);

            result
        }
    }
}

impl std::error::Error for PlatformError {}

impl FallibleContext for super::Context {
    type PlatformError = PlatformError;
}
