pub struct Convert;

impl Convert {
    pub const SAMPLE_RATE: u64 = 16_000;
    pub const SAMPLE_WIDTH_FLOAT32: u64 = 4;
    pub const SAMPLE_WIDTH_INT16: u64 = 2;

    pub fn float32_to_ms(buffer_length: u64) -> u64 {
        buffer_length / Self::SAMPLE_RATE / Self::SAMPLE_WIDTH_FLOAT32 * 1000
    }

    pub fn ms_to_float32(ms: u64) -> u64 {
        ms * Self::SAMPLE_RATE * Self::SAMPLE_WIDTH_FLOAT32 / 1000
    }

    pub fn int16_to_ms(buffer_length: u64) -> u64 {
        buffer_length / Self::SAMPLE_RATE / Self::SAMPLE_WIDTH_INT16 * 1000
    }

    pub fn ms_to_int16(ms: u64) -> u64 {
        ms * Self::SAMPLE_RATE * Self::SAMPLE_WIDTH_INT16 / 1000
    }
}
