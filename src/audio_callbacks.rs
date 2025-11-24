use sdl2::audio::AudioCallback;

// For u8 audio
pub struct U8AudioCallback {
    pub bytes: Vec<u8>,
    pub position: usize,
}

impl AudioCallback for U8AudioCallback {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let remaining = self.bytes.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.bytes[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

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
        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.samples[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

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
        let remaining = self.samples.len().saturating_sub(self.position);
        let to_copy = data.len().min(remaining);

        if to_copy > 0 {
            data[..to_copy].copy_from_slice(&self.samples[self.position..self.position + to_copy]);
            self.position += to_copy;
        }

        if to_copy < data.len() {
            data[to_copy..].fill(0);
        }
    }
}
