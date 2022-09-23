pub enum VisCode {
    Scottie1 = 60,
    Scottie2 = 56,
    ScottieDX = 76,
    Martin1 = 44,
    Martin2 = 40,
}

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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

pub struct Context {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    color: Color,
    curr_hz: f32,
    curr_ampl: f32,
    time_ms: f32,
    oscil_time: f32,
}

impl Context {
    pub fn new() -> Self {
        Context { 
            width: 320,
            height: 256,
            x : 0,
            y: 0,
            color: Color::new(),
            curr_hz: 0.0,
            curr_ampl: 0.0,
            time_ms: 0.0,
            oscil_time: 0.0,
        }
    }
}

pub struct SilenceState {
    name: &'static str,
    ms: f32,
}

impl SilenceState {
    pub fn new(name: &'static str, ms: f32) -> Box<Self> {
        Box::new(SilenceState { name: name, ms: ms })
    }
}

pub struct ToneState {
    name: &'static str,
    ms: f32,
    hz: f32,
}

impl ToneState {
    pub fn new(name: &'static str, ms: f32, hz: f32) -> Box<Self> {
        Box::new(ToneState { name: name, ms: ms, hz: hz })
    }
}

pub struct ColorRGBScanState {
    name: &'static str,
    ms: f32,
    ch: u32
}

impl ColorRGBScanState {
    pub fn new(name: &'static str, ms: f32, ch: u32) -> Box<Self> {
        Box::new(ColorRGBScanState { name: name, ms: ms, ch: ch })
    }
}

pub trait EncodeState {
    fn encode(&self, ctx: &mut Context);
}

impl EncodeState for SilenceState {
    fn encode(&self, ctx: &mut Context) {
        println!("Encode {}, {}ms", self.name, self.ms);
        ctx.curr_hz = 1.0;
        ctx.curr_ampl = 0.0;
        ctx.oscil_time = 0.0;
        // check_end_state()
    }
}

impl EncodeState for ToneState {
    fn encode(&self, ctx: &mut Context) {
        println!("Encode {}, {}ms, {}hz", self.name, self.ms, self.hz);
        ctx.curr_hz = self.hz;
        ctx.curr_ampl = 1.0;
        // check_end_state()
    }
}

impl EncodeState for ColorRGBScanState {
    fn encode(&self, ctx: &mut Context) {
        println!("Encode {}, {}ms, {}ch", self.name, self.ms, self.ch);
        ctx.x = (ctx.time_ms / self.ms / (ctx.width as f32)) as u32;
        ctx.curr_hz = 1500.0 + (2300.0 - 1500.0) * ctx.color.get_ch(self.ch);
        ctx.curr_ampl = 1.0;
        if ctx.color.a < 0.5 {
            ctx.curr_ampl = 0.0;
        }

    }
}
