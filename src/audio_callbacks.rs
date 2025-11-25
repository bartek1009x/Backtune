use sdl2::audio::AudioCallback;

// For u8 audio
pub struct U8AudioCallback {
    pub bytes: Vec<u8>,
    pub position: usize,
}

impl AudioCallback for U8AudioCallback {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let vol = crate::settings::get_cloned_settings().volume as f32;

        let remaining = self.bytes.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        for i in 0..to_copy {
            let raw = self.bytes[self.position + i] as f32;

            let centered = raw - 128.0;
            let scaled = centered * vol;
            data[i] = ((scaled + 128.0).clamp(0.0, 255.0)) as u8;
        }

        self.position += to_copy;

        if to_copy < data.len() {
            data[to_copy..].fill(128);
        }
    }
}

// For i16 audio
pub struct I16AudioCallback {
    pub samples: Vec<i16>,
    pub position: usize,
}

impl AudioCallback for I16AudioCallback {
    type Channel = i16;

    fn callback(&mut self, data: &mut [i16]) {
        let vol = crate::settings::get_cloned_settings().volume as f32;

        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        for i in 0..to_copy {
            let raw = self.samples[self.position + i] as f32;
            let scaled = (raw * vol).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            data[i] = scaled;
        }

        self.position += to_copy;

        if to_copy < data.len() {
            data[to_copy..].fill(0);
        }
    }
}

// For i32 audio
pub struct I32AudioCallback {
    pub samples: Vec<i32>,
    pub position: usize,
}

impl AudioCallback for I32AudioCallback {
    type Channel = i32;

    fn callback(&mut self, data: &mut [i32]) {
        let vol = crate::settings::get_cloned_settings().volume as f32;

        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        for i in 0..to_copy {
            let raw = self.samples[self.position + i] as f32;
            let scaled = (raw * vol).clamp(i32::MIN as f32, i32::MAX as f32) as i32;
            data[i] = scaled;
        }

        self.position += to_copy;

        if to_copy < data.len() {
            data[to_copy..].fill(0);
        }
    }
}
