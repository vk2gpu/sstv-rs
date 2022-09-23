use crate::state::*;

pub struct EncodeContext {
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

impl EncodeContext {
    pub fn new() -> Self {
        EncodeContext { 
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
pub trait EncodeState {
    fn encode(&self, ctx: &mut EncodeContext);
}

impl EncodeState for SilenceState {
    fn encode(&self, ctx: &mut EncodeContext) {
        println!("Encode {}, {}ms", self.name, self.ms);
        ctx.curr_hz = 1.0;
        ctx.curr_ampl = 0.0;
        ctx.oscil_time = 0.0;
        // check_end_state()
    }
}

impl EncodeState for ToneState {
    fn encode(&self, ctx: &mut EncodeContext) {
        println!("Encode {}, {}ms, {}hz", self.name, self.ms, self.hz);
        ctx.curr_hz = self.hz;
        ctx.curr_ampl = 1.0;
        // check_end_state()
    }
}

impl EncodeState for ColorRGBScanState {
    fn encode(&self, ctx: &mut EncodeContext) {
        println!("Encode {}, {}ms, {}ch", self.name, self.ms, self.ch);
        ctx.x = (ctx.time_ms / self.ms / (ctx.width as f32)) as u32;
        ctx.curr_hz = 1500.0 + (2300.0 - 1500.0) * ctx.color.get_ch(self.ch);
        ctx.curr_ampl = 1.0;
        if ctx.color.a < 0.5 {
            ctx.curr_ampl = 0.0;
        }

    }
}


fn encode() {
    println!("encode");
}
