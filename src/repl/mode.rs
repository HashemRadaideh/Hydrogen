/// Enum representing different cursor modes in the Read-Eval-Print Loop (REPL).
pub enum CursorMode {
    /// Normal cursor mode.
    Normal,
    /// Vi cursor mode.
    Vi,
    /// Emacs cursor mode.
    Emacs,
}

impl CursorMode {
    /// Creates a new `CursorMode` based on the specified mode string.
    ///
    /// # Arguments
    ///
    /// * `mode` - The cursor mode as a string ("normal", "vi", or "emacs").
    ///
    /// # Returns
    ///
    /// * `Self` - A new `CursorMode` instance.
    pub fn new(mode: String) -> Self {
        if mode == "normal" {
            CursorMode::Normal
        } else if mode == "vi" {
            CursorMode::Vi
        } else if mode == "emacs" {
            CursorMode::Emacs
        } else {
            // Default to Normal cursor mode if an invalid mode is provided.
            CursorMode::Normal
        }
    }
}
