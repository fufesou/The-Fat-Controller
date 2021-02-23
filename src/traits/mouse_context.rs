use crate::{Error, MouseButton};

/// A context that supports mouse events.
///
/// # Platform Differences
///
/// On Linux, smooth scrolling isn't supported so
/// [`mouse_scroll`](MouseContext::mouse_scroll) will accumulate up to `120` (a
/// magic number that seems to pop up in various places) before issuing a scroll
/// event.
pub trait MouseContext {

    /// Move the mouse relative to its current location.
    ///
    /// # Arguments
    ///
    /// * `dx` - The horizontal offset. Positive values move to the right and
    /// negative values move to the left.
    /// * `dy` - The vertical offset. Positive values move down and negative
    /// values move up.
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error>;

    /// Move the mouse to an absolute location.
    ///
    /// # Arguments
    ///
    /// * `x` - The horizontal position. A zero value is the left side of the
    /// screen.
    /// * `y` - The vertical position. A zero value is the top of the screen.
    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error>;

    /// Scroll the mouse horizontally and vertically in pixels.
    ///
    /// # Arguments
    ///
    /// * `dx` - The horizontal offset. Positive values scroll to the right and
    /// negative values scroll to the left.
    /// * `dy` - The vertical offset. Positive values scroll down and negative
    /// values scroll up.
    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error>;

    /// Press down a mouse button.
    ///
    /// # Arguments
    ///
    /// * `button` - The button to press down.
    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error>;

    /// Release a mouse button.
    ///
    /// # Arguments
    ///
    /// * `button` - The button to release.
    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error>;

    /// Press and release a mouse button.
    ///
    /// This is equivalent to calling [`mouse_down`](MouseContext::mouse_down)
    /// followed by [`mouse_up`](MouseContext::mouse_up).
    ///
    /// # Arguments
    ///
    /// * `button` - The button to press and release.
    fn mouse_click(&mut self, button: MouseButton) -> Result<(), Error> {
        self.mouse_down(button)?;
        self.mouse_up(button)
    }
}