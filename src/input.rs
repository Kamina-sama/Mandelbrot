pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub exit: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            zoom_in: false,
            zoom_out: false,
            exit: false,
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            zoom_in: false,
            zoom_out: false,
            exit: false,
        }
    }
}