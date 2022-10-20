#[derive(Clone, Copy)]
pub enum VisCode {
    Scottie1 = 60,
    Scottie2 = 56,
    ScottieDX = 76,
    Martin1 = 44,
    Martin2 = 40,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    pub fn get_ch(&self, ch: u32) -> f32 {
        match ch {
            0 => return self.r,
            1 => return self.g,
            2 => return self.b,
            3 => return self.a,
            _ => panic!("Invalid channel"),
        }
    }
}

pub struct SilenceState {
    pub name: &'static str,
    pub ms: f32,
    pub next_state: i32,
}

impl SilenceState {
    pub fn new(name: &'static str, ms: f32, next_state: i32) -> Box<Self> {
        Box::new(SilenceState { name: name, ms: ms, next_state: next_state })
    }
}

pub struct ToneState {
    pub name: &'static str,
    pub ms: f32,
    pub hz: f32,
    pub next_state: i32,
}

impl ToneState {
    pub fn new(name: &'static str, ms: f32, hz: f32, next_state: i32) -> Box<Self> {
        Box::new(ToneState { name: name, ms: ms, hz: hz, next_state: next_state})
    }
}

pub struct ColorRGBScanState {
    pub name: &'static str,
    pub ms: f32,
    pub ch: u32,
    pub next_state: i32,
}

impl ColorRGBScanState {
    pub fn new(name: &'static str, ms: f32, ch: u32, next_state: i32) -> Box<Self> {
        Box::new(ColorRGBScanState { name: name, ms: ms, ch: ch, next_state: next_state })
    }
}
