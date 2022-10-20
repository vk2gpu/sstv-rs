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

fn check_end_state(ctx: &mut EncodeContext, state: &dyn EncodeState) {
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

fn count_bits_set(in_value: u32) -> u32
{
    let mut value = in_value; 
    value = (value & 0x55555555) + ((value & 0xAAAAAAAA) >> 1);
    value = (value & 0x33333333) + ((value & 0xCCCCCCCC) >> 2);
    value = (value & 0x0F0F0F0F) + ((value & 0xF0F0F0F0) >> 4);
    value = (value & 0x00FF00FF) + ((value & 0xFF00FF00) >> 8);
    return ((value & 0x0000FFFF) + ((value & 0xFFFF0000) >> 16)) as u32;
}


fn get_states_preamble() -> EncodeStates {
    return vec![
        ToneState::new("Preamble", 100.0, 1900.0, 1),
        ToneState::new("Preamble", 100.0, 1500.0, 1),
        ToneState::new("Preamble", 100.0, 1900.0, 1),
        ToneState::new("Preamble", 100.0, 1500.0, 1),
        ToneState::new("Preamble", 100.0, 2300.0, 1),
        ToneState::new("Preamble", 100.0, 1500.0, 1),
        ToneState::new("Preamble", 100.0, 2300.0, 1),
        ToneState::new("Preamble", 100.0, 1500.0, 1),
    ];
}

fn get_states_vis_code(vis_code: VisCode) -> EncodeStates {
    let vis_code_bits = vis_code as u32;
    let bits: &[f32] = &[
        if vis_code_bits & 0x01 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x02 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x04 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x08 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x10 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x20 != 0 { 1100.0 } else { 1300.0 },
        if vis_code_bits & 0x40 != 0 { 1100.0 } else { 1300.0 },
        if count_bits_set(vis_code_bits) & 1 != 0 { 1100.0 } else { 1300.0 },
    ];

    return vec![
        ToneState::new("Leader Tone", 300.0, 1900.0, 1),
        ToneState::new("Break", 10.0, 1200.0, 1),
        ToneState::new("Leader Tone", 300.0, 1900.0, 1),
        ToneState::new("Start Bit", 30.0, 1200.0, 1),
        ToneState::new("Bit 0", 30.0, bits[0], 1),
        ToneState::new("Bit 1", 30.0, bits[1], 1),
        ToneState::new("Bit 2", 30.0, bits[2], 1),
        ToneState::new("Bit 3", 30.0, bits[3], 1),
        ToneState::new("Bit 4", 30.0, bits[4], 1),
        ToneState::new("Bit 5", 30.0, bits[5], 1),
        ToneState::new("Bit 6", 30.0, bits[6], 1),
        ToneState::new("Parity", 30.0,bits[7], 1),
        ToneState::new("Stop Bit", 30.0, 1200.0, 1),
    ];
}


fn get_states_scottie(vis_code: VisCode) -> EncodeStates {
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

fn get_states_martin(vis_code: VisCode) -> EncodeStates {
    let scan_ms: f32 = match vis_code {
        VisCode::Martin1 => 146.432,
        VisCode::Martin2 => 73.216,
        _ => panic!("Invalid vis mode, expected Martin."),
    };

    return vec![
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

pub fn get_states(vis_code: VisCode) -> EncodeStates {
    let preamble_states = get_states_preamble();
    let vis_code_states = get_states_vis_code(vis_code);
    let mode_states = match vis_code {
        VisCode::Scottie1 => get_states_scottie(vis_code),
        VisCode::Scottie2 => get_states_scottie(vis_code),
        VisCode::ScottieDX => get_states_scottie(vis_code),
        VisCode::Martin1 => get_states_martin(vis_code),
        VisCode::Martin2 => get_states_martin(vis_code),
        _ => panic!("Invalid vis mode."),
    };

    let mut out_states = EncodeStates::new();
    out_states.extend(preamble_states);
    out_states.extend(vis_code_states);
    out_states.extend(mode_states);
    return out_states;
}

pub fn encode(states: &EncodeStates, input: &mut dyn EncodeInput, output: &mut dyn EncodeOutput) {
    let mut ctx = EncodeContext::new();

    let mut curr_state_idx: i32 = 0;

    let time_ms_adv: f32 = 1000.0 / ctx.sample_rate;
    let mut curr_ampl: f32 = 1.0;

    let mut pct = 0;
    let mut curr_time = 0.0;

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
