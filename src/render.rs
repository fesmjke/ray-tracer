pub enum Rendering {
    Parallel,
    Sequential,
}

pub struct Render {
    pub render_mode: Rendering,
}

impl Render {
    pub fn new() -> Self {
        Self {
            render_mode: Rendering::Parallel,
        }
    }
}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}
