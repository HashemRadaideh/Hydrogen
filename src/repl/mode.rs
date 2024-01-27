pub enum CursorMode {
    Normal,
    Vi,
    Emacs,
}

impl CursorMode {
    pub fn new(mode: String) -> Self {
        if mode == "normal" {
            CursorMode::Normal
        } else if mode == "vi" {
            CursorMode::Vi
        } else if mode == "emacs" {
            CursorMode::Emacs
        } else {
            CursorMode::Normal
        }
    }
}
