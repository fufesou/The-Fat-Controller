use super::FallibleContext;
use crate::GenericError;

/// A context that supports layout-independent Unicode keyboard events.
///
/// # Platform Differences
///
/// This trait is not implemented for Linux-Wayland.
/// [`AsciiKeyboardContext`](crate::AsciiKeyboardContext) may be used as an
/// alternative. For convenience, the `"ascii-fallback"` feature may be enabled
/// to provide an implementation of `UnicodeKeyboardContext` that uses
/// [`AsciiKeyboardContext`](crate::AsciiKeyboardContext).
///
/// On macOS and Windows,
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) is not equivalent
/// to successive calls to
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char).
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) is meant to press a
/// key corresponding to a character which means that modifiers can be applied.
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) is meant to type
/// an arbitrary Unicode string (possibly bypassing the keyboard) meaning that
/// modifiers cannot be applied. In short, the two functions serve different
/// purposes.
///
/// The following snippet will do a select-all on any keyboard layout. Note that
/// we're using a generic function here because
/// [`UnicodeKeyboardContext`](UnicodeKeyboardContext) is not implemented when
/// compiling for `docs.rs`.
/// ```no_run
/// use tfc::{GenericError, Key, traits::*};
///
/// fn select_all<C>(ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
///     where C: KeyboardContext + UnicodeKeyboardContext + FallibleContext
/// {
///     ctx.key_down(Key::ControlOrMeta)?;
///     ctx.unicode_char('a')?;
///     ctx.key_up(Key::ControlOrMeta)
/// }
/// ```
///
/// However, the next snippet will only do a select-all on Linux-X11.
/// ```no_run
/// use tfc::{GenericError, Key, traits::*};
///
/// fn select_all<C>(ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
///     where C: KeyboardContext + UnicodeKeyboardContext + FallibleContext
/// {
///     ctx.key_down(Key::ControlOrMeta)?;
///     ctx.unicode_string("a")?;
///     ctx.key_up(Key::ControlOrMeta)
/// }
/// ```
///
/// Care must be taken when using
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) in this manner. If an
/// uppercase `'A'` was used, the shift key would have been pressed which may
/// not have had the desired effect.
///
/// On Windows, [`unicode_char`](UnicodeKeyboardContext::unicode_char) is
/// limited to characters that can be represented as a single UTF-16 code unit
/// and are on the current keyboard layout. However,
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) doesn't have
/// these restrictions and can handle any Unicode string.
///
/// Similarly, on macOS, [`unicode_char`](UnicodeKeyboardContext::unicode_char)
/// is limited to characters that are on the current keyboard layout (which may
/// include some fancy characters like  and ©) but again,
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) can handle any
/// Unicode string.
pub trait UnicodeKeyboardContext: FallibleContext {
    /// Generate a key press event (possibly including modifiers) for a Unicode
    /// character.
    ///
    /// If modifiers are necessary to type the character, then the modifiers
    /// will be pressed.
    ///
    /// Returns [`UnsupportedUnicode`](GenericError::UnsupportedUnicode) if the
    /// given character is unsupported.
    fn unicode_char_down(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>>;

    /// Generate a key release event (possibly including modifiers) for a
    /// Unicode character.
    ///
    /// If modifiers are necessary to type the character, then the modifiers
    /// will be released.
    ///
    /// Returns [`UnsupportedUnicode`](GenericError::UnsupportedUnicode) if the
    /// given character is unsupported.
    fn unicode_char_up(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>>;

    /// Generate a key press and release event along with the necessary
    /// modifiers to type a unicode character.
    ///
    /// This is equivalent to calling
    /// [`unicode_char_down`](UnicodeKeyboardContext::unicode_char_down)
    /// followed by
    /// [`unicode_char_up`](UnicodeKeyboardContext::unicode_char_up).
    ///
    /// Returns [`UnsupportedUnicode`](GenericError::UnsupportedUnicode) if the
    /// given character is unsupported.
    fn unicode_char(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>>;

    /// Generate key presses and releases such that a Unicode string is typed.
    ///
    /// If any of the characters in the string are unsupported,
    /// [`UnsupportedUnicode`](GenericError::UnsupportedUnicode) will be
    /// returned and no key presses will occur.
    fn unicode_string(&mut self, s: &str) -> Result<(), GenericError<Self::PlatformError>>;
}
