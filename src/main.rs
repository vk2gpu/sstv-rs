mod encoder;
mod state;

use encoder::*;
use state::*;

extern crate sndfile;
use sndfile::*;

extern crate image;
use image::*;


struct EncodeInputTest {

}

impl EncodeInputTest {
    pub fn new() -> Self {
        EncodeInputTest{}
    }
}

impl EncodeInput for EncodeInputTest {
    fn read(&self, x: f32, y:f32) -> Color {
        Color {
            r: x,
            g: y,
            b: 0.0,
            a: 1.0
        }
    }
}

struct EncodeInputImage {
    image: image::Rgba32FImage,
}

impl EncodeInputImage {
    fn new(file_name: &str) -> Self {
        EncodeInputImage{
            image: image::open(&file_name)
                .expect("Open file failed")
                .to_rgba32f()
        }
    }
}

impl EncodeInput for EncodeInputImage {
    fn read(&self, x: f32, y:f32) -> Color {
        let ax = (x * (self.image.width() as f32 - 1.0)) as u32;
        let ay = (y * (self.image.height() as f32 - 1.0)) as u32;
        let px = self.image.get_pixel(ax, ay);
        let ch = px.channels();
        Color {
            
            r: ch[0],
            g: ch[1],
            b: ch[2],
            a: ch[3],
        }
    }
}

struct EncodeOutputSndFile {
    snd_file: sndfile::SndFile,
    buffer: Vec<f32>,
}


impl EncodeOutputSndFile {
    pub fn new(file_name: &str) -> Self {
        let write_options = sndfile::WriteOptions::new(
            sndfile::MajorFormat::WAV,
            sndfile::SubtypeFormat::PCM_16,
            sndfile::Endian::Little,
            8000,
            1,
        );

        let snd_file = sndfile::OpenOptions::WriteOnly(write_options).from_path(
            file_name
          ).unwrap();

        let buffer = Vec::new();
        
        return EncodeOutputSndFile{ snd_file, buffer }
    }
}

impl Drop for EncodeOutputSndFile {
    fn drop(&mut self) {
        let result = self.snd_file.write_from_slice(self.buffer.as_slice());
        result.unwrap();
    }
}

impl EncodeOutput for EncodeOutputSndFile {
    fn write(&mut self, value: f32) -> usize{
        self.buffer.push(value);
        return 1;
    }
}

fn main() {
    let mut output = EncodeOutputSndFile::new("test.wav");
    let mut input = EncodeInputImage::new("test_image.jpg");

    let states: EncodeStates = get_states(VisCode::Martin2);

    encode(&states, &mut input, &mut output);
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



