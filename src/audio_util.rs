pub mod AudioUtil {
    #[inline]
    pub fn convert_f_to_i32(in_buf: &Vec<u8>, out_buf: &mut Vec<i32>) {
        assert_eq!(in_buf.len() % 4, 0);
        if in_buf.len() / 4 != out_buf.len() {
            out_buf.resize(in_buf.len() / 4, 0);
        }
        for (i, c) in in_buf.chunks_exact(4).enumerate() {
            let f32_buf = f32::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = (f32_buf * (1 << 24) as f32) as i32;
        }
    }

    #[inline]
    pub fn convert_f_to_i16(in_buf: &Vec<u8>, out_buf: &mut Vec<i16>) {
        assert_eq!(in_buf.len() % 4, 0);
        if in_buf.len() / 4 != out_buf.len() {
            out_buf.resize(in_buf.len() / 4, 0);
        }
        for (i, c) in in_buf.chunks(4).enumerate() {
            let f32_buf = f32::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = (f32_buf * i16::MAX as f32) as i16;
        }
    }

    #[inline]
    pub fn convert_i32_to_f(in_buf: &Vec<u8>, out_buf: &mut Vec<f32>) {
        assert_eq!(in_buf.len() % 4, 0);
        if in_buf.len() / 4 != out_buf.len() {
            out_buf.resize(in_buf.len() / 4, 0.);
        }
        let factor = 1.0_f32 / (1 << 24) as f32;
        for (i, c) in in_buf.chunks(4).enumerate() {
            let i32_buf = i32::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = i32_buf as f32 * factor;
        }
    }

    #[inline]
    pub fn convert_i32_to_i16(in_buf: &Vec<u8>, out_buf: &mut Vec<i16>) {
        assert_eq!(in_buf.len() % 4, 0);
        if in_buf.len() / 4 != out_buf.len() {
            out_buf.resize(in_buf.len() / 4, 0);
        }
        for (i, c) in in_buf.chunks(4).enumerate() {
            let i32_buf = i32::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = (i32_buf >> 9) as i16;
        }
    }

    #[inline]
    pub fn convert_i16_to_f(in_buf: &Vec<u8>, out_buf: &mut Vec<f32>) {
        assert_eq!(in_buf.len() % 2, 0);
        if in_buf.len() / 2 != out_buf.len() {
            out_buf.resize(in_buf.len() / 2, 0.);
        }
        let factor = 1.0_f32 / i16::MAX as f32;
        for (i, c) in in_buf.rchunks(2).enumerate() {
            let i16_buf = i16::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = i16_buf as f32 * factor;
        }
    }

    #[inline]
    pub fn convert_i16_to_i32(in_buf: &Vec<u8>, out_buf: &mut Vec<i32>) {
        assert_eq!(in_buf.len() % 2, 0);
        if in_buf.len() / 2 != out_buf.len() {
            out_buf.resize(in_buf.len() / 2, 0);
        }
        for (i, c) in in_buf.rchunks(2).enumerate() {
            let i16_buf = i16::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = (i16_buf as i32) << 9;
        }
    }

    #[inline]
    pub fn convert_i8_f(in_buf: &Vec<u8>, out_buf: &mut Vec<f32>) {
        assert_eq!(out_buf.len(), in_buf.len());
        let factor = 1.0_f32 / i8::MAX as f32;
        for (i, c) in in_buf.iter().rev().enumerate() {
            out_buf[i] = i8::from_le_bytes(*std::array::from_ref(c)) as f32 * factor;
        }
    }

    #[inline]
    pub fn convert_i8_to_i32(in_buf: &Vec<u8>, out_buf: &mut Vec<i32>) {
        assert_eq!(out_buf.len(), in_buf.len());
        for (i, c) in in_buf.iter().rev().enumerate() {
            out_buf[i] = (i8::from_le_bytes(*std::array::from_ref(c)) as i32) << 17;
        }
    }

    #[inline]
    pub fn convert_f_to_f(in_buf: &Vec<u8>, out_buf: &mut Vec<f32>) {
        assert_eq!(in_buf.len() % 4, 0);
        if in_buf.len() / 4 != out_buf.len() {
            out_buf.resize(in_buf.len() / 4, 0.);
        }
        for (i, c) in in_buf.chunks(4).enumerate() {
            let f32_buf = f32::from_le_bytes(c.try_into().unwrap());
            out_buf[i] = f32_buf;
        }
    }
}
