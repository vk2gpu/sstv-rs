mod modes;

use modes::*;

fn main() {
    let scan_ms = 138.24; // scottie 1

    let mut states: Vec<Box<dyn EncodeState>> = vec![
        ToneState::new("Starting Sync Pulse", 9.0, 1200.0),
        ToneState::new("Separator Pulse", 1.5, 1500.0),
        ColorRGBScanState::new("Green Scan", scan_ms, 1),
        ToneState::new("Separator Pulse", 1.5, 1500.0),
        ColorRGBScanState::new("Blue Scan", scan_ms, 2),
        ToneState::new("Sync Pulse", 9.0, 1200.0),
        ToneState::new("Sync Porch", 1.5, 1500.0),
        ColorRGBScanState::new("Red Scan", scan_ms, 0),
    ];
    
    let mut ctx = Context::new();

    for state in states {
        state.encode(&mut ctx);
    }
}


/*
state_t* mode_scottie(context_t* ctx, state_t* states, uint32_t vis)
{  
    float scan_ms = 0.0f;
    switch(vis) {
        case VIS_CODE_SCOTTIE_1: scan_ms = 138.24f; break;
        case VIS_CODE_SCOTTIE_2: scan_ms = 88.064f; break;
        case VIS_CODE_SCOTTIE_DX: scan_ms = 345.6f; break;
        default: return 0;
    }

    ctx->width = 320;
    ctx->height = 256;

    state_tone( &states[0], 9.0f, 1200.0f, &states[1] );            // “Starting” sync pulse (first line only!) 9.0ms 1200hz
    state_tone( &states[1], 1.5f, 1500.0f, &states[2] );            // Separator pulse 1.5ms 1500hz
    state_rgb_color_scan( &states[2], scan_ms, 1, &states[3] );     // Green scan
    state_tone( &states[3], 1.5f, 1500.0f, &states[4] );            // Separator pulse 1.5ms 1500hz
    state_rgb_color_scan( &states[4], scan_ms, 2, &states[5] );     // Blue scan
    state_tone( &states[5], 9.0f, 1200.0f, &states[6] );            // Sync pulse 9.0ms 1200hz
    state_tone( &states[6], 1.5f, 1500.0f, &states[7] );            // Sync porch 1.5ms 1500hz
    state_rgb_color_scan( &states[7], scan_ms, 0, &states[1] );     // Red scan
    ctx->end_state = &states[7];
    return ctx->end_state + 1;
}

 */



