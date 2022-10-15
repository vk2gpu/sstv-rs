use std::f32::consts::{TAU};

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
    sample_rate: f32,
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
            sample_rate: 8000.0
        }
    }
}
pub trait EncodeState {
    fn encode(&self, ctx: &mut EncodeContext);
    fn get_ms(&self) -> f32;
    fn get_next_state(&self) -> i32;
}

fn check_end_state(ctx: &mut EncodeContext, state: &EncodeState) {
    if state.get_next_state() < 0 && ctx.time_ms >= state.get_ms() {
        ctx.y += 1;
        //if(ctx.y >= ctx->height)
        //    state->nextState = state + 1;
    }
}

impl EncodeState for SilenceState {
    fn encode(&self, ctx: &mut EncodeContext) {
        ctx.curr_hz = 1.0;
        ctx.curr_ampl = 0.0;
        ctx.oscil_time = 0.0;
        check_end_state(ctx, self);
    }

    fn get_ms(&self) -> f32 {
        self.ms
    }

    fn get_next_state(&self) -> i32 {
        self.next_state
    }
}

impl EncodeState for ToneState {
    fn encode(&self, ctx: &mut EncodeContext) {
        ctx.curr_hz = self.hz;
        ctx.curr_ampl = 1.0;
        check_end_state(ctx, self);
    }

    fn get_ms(&self) -> f32 {
        self.ms
    }

    fn get_next_state(&self) -> i32 {
        self.next_state
    }
}

impl EncodeState for ColorRGBScanState {
    fn encode(&self, ctx: &mut EncodeContext) {
        ctx.x = (ctx.time_ms / (self.ms / (ctx.width as f32))) as u32;
        ctx.curr_hz = 1500.0 + (2300.0 - 1500.0) * ctx.color.get_ch(self.ch);
        ctx.curr_ampl = 1.0;
        if ctx.color.a < 0.5 {
            ctx.curr_ampl = 0.0;
        }
        check_end_state(ctx, self);
    }

    fn get_ms(&self) -> f32 {
        self.ms
    }

    fn get_next_state(&self) -> i32 {
        self.next_state
    }
}

pub trait EncodeInput {
    fn read(&self, x: f32, y:f32) -> Color;
}

pub trait EncodeOutput {
    fn write(&mut self, value: f32) -> usize;
}

pub type EncodeStateBoxed = Box<dyn EncodeState>;
pub type EncodeStates = Vec<EncodeStateBoxed>;

fn getStatesScottie(vis_code: VisCode) -> EncodeStates {
    let scan_ms: f32 = match vis_code {
        VisCode::Scottie1 => 138.24,
        VisCode::Scottie2 => 88.064,
        VisCode::ScottieDX => 345.6,
        _ => panic!("Invalid vis mode, expected Scottie."),
    };

    return vec![
        SilenceState::new("Start Silence", 500.0, 1),
        ToneState::new("Starting Sync Pulse", 9.0, 1200.0, 1),
        ToneState::new("Separator Pulse", 1.5, 1500.0, 1),
        ColorRGBScanState::new("Green Scan", scan_ms, 1, 1),
        ToneState::new("Separator Pulse", 1.5, 1500.0, 1),
        ColorRGBScanState::new("Blue Scan", scan_ms, 2, 1),
        ToneState::new("Sync Pulse", 9.0, 1200.0, 1),
        ToneState::new("Sync Porch", 1.5, 1500.0, 1),
        ColorRGBScanState::new("Red Scan", scan_ms, 0, -6),
    ];
}

/*
    float scan_ms = 0.0f;
    switch(vis) {
        case VIS_CODE_MARTIN_1: scan_ms = 146.432f; break;
        case VIS_CODE_MARTIN_2: scan_ms = 73.216f; break;
        default: return 0;
    }

    ctx->width = 320;
    ctx->height = 256;

    state_tone( &states[0], 4.862f, 1200.0f, &states[1] );          // Sync pulse 
    state_tone( &states[1], 0.572f, 1500.0f, &states[2] );          // Sync porch
    state_rgb_color_scan( &states[2], scan_ms, 1, &states[3] );     // Green scan
    state_tone( &states[3], 0.572f, 1500.0f, &states[4] );          // Separator pulse
    state_rgb_color_scan( &states[4], scan_ms, 2, &states[5] );     // Blue scan
    state_tone( &states[5], 0.572f, 1500.0f, &states[6] );          // Separator pulse
    state_rgb_color_scan( &states[6], scan_ms, 0, &states[7] );     // Red scan
    state_tone( &states[7], 0.572f, 1500.0f, &states[0] );          // Separator pulse
    ctx->end_state = &states[7];
    return ctx->end_state + 1;

    */

fn getStatesMartin(vis_code: VisCode) -> EncodeStates {
    let scan_ms: f32 = match vis_code {
        VisCode::Martin1 => 146.432,
        VisCode::Martin2 => 73.216,
        _ => panic!("Invalid vis mode, expected Martin."),
    };

    return vec![
        SilenceState::new("Start Silence", 500.0, 1),
        ToneState::new("Sync Pulse", 4.862, 1200.0, 1),
        ToneState::new("Sync Porch", 0.572, 1500.0, 1),
        ColorRGBScanState::new("Green Scan", scan_ms, 1, 1),
        ToneState::new("Separator Pulse", 0.572, 1500.0, 1),
        ColorRGBScanState::new("Blue Scan", scan_ms, 2, 1),
        ToneState::new("Separator Pulse", 0.572, 1500.0, 1),
        ColorRGBScanState::new("Red Scan", scan_ms, 0, 1),
        ToneState::new("Separator Pulse", 0.572, 1500.0, -7),
    ];
}

pub fn getStates(vis_code: VisCode) -> EncodeStates {
    return match vis_code {
        VisCode::Scottie1 => getStatesScottie(vis_code),
        VisCode::Scottie2 => getStatesScottie(vis_code),
        VisCode::ScottieDX => getStatesScottie(vis_code),
        VisCode::Martin1 => getStatesMartin(vis_code),
        VisCode::Martin2 => getStatesMartin(vis_code),
        _ => panic!("Invalid vis mode."),
    };
}

pub fn encode(states: &EncodeStates, input: &mut dyn EncodeInput, output: &mut dyn EncodeOutput) {
    let mut ctx = EncodeContext::new();

    let mut curr_state_idx: i32 = 0;

    let time_ms_adv: f32 = 1000.0 / ctx.sample_rate;
    let mut curr_ampl: f32 = 1.0;

    let mut pct = 0;
    let mut curr_time = 0.0;
    let max_time = 60000.0;

    while ctx.y < ctx.height {
        ctx.color = input.read(ctx.x as f32 / ctx.width as f32, ctx.y as f32 / ctx.height as f32 );

        let curr_state = &states[curr_state_idx as usize];
        curr_state.encode(&mut ctx);

        if ctx.time_ms >= curr_state.get_ms() {
            ctx.time_ms -= curr_state.get_ms();
            curr_state_idx = curr_state_idx + curr_state.get_next_state();
        }

        let oscil_adv: f32 = ctx.curr_hz / ctx.sample_rate; 
        ctx.oscil_time = (ctx.oscil_time + oscil_adv) % 1.0;
        curr_ampl = curr_ampl * 0.9 + ctx.curr_ampl * 0.1;
        let ampl = f32::sin(ctx.oscil_time * TAU);

        output.write(ampl);

        ctx.time_ms += time_ms_adv;
        curr_time += time_ms_adv;

        let new_pct = ((ctx.y as f32 / ctx.height as f32) * 100.0) as i32;
        if new_pct != pct {
            pct = new_pct;
            println!("{} complete..({}ms)", pct, curr_time);
        }
    }

}
