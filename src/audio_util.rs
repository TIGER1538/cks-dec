pub(crate) struct AudioUtil {
    four_bytes: [u8; 4],
    two_bytes: [u8; 2],
}

impl AudioUtil {
    pub fn new() -> Self {
        Self {
            four_bytes: [0u8; 4],
            two_bytes: [0u8; 2],
        }
    }

    pub fn convert_f_to_i32(&mut self, in_buf: &[u8], out_buf: &mut [i32]) {
        let mut _buf = [0_u8; 4];
        //assert_eq!(out_buf.len(), in_buf.len());
        for i in 0..in_buf.len() / 4 {
            _buf.copy_from_slice(&in_buf[4 * i..4 * i + 4]);
            let f32_buf = f32::from_be_bytes(_buf);
            out_buf[i] = (f32_buf * (1 << 24) as f32) as i32;
        }
    }

    pub fn convert_f_to_i16(&mut self, in_buf: &[u8], out_buf: &mut [i16]) {
        let mut _buf = [0_u8; 4];
        //assert_eq!(out_buf.len(), in_buf.len());
        for i in 0..in_buf.len() / 4 {
            _buf.copy_from_slice(&in_buf[4 * i..4 * i + 4]);
            let f32_buf = f32::from_be_bytes(_buf);
            out_buf[i] = (f32_buf * i16::MAX as f32) as i16;
        }
    }

    pub fn convert_i32_to_f(&mut self, in_buf: &[u8], out_buf: &mut [f32]) {
        let mut _buf = [0_u8; 4];
        //assert_eq!(out_buf.len(), in_buf.len());
        let factor = 1.0_f32 / (1 << 24) as f32;
        for i in 0..in_buf.len() / 4 {
            _buf.copy_from_slice(&in_buf[4 * i..4 * i + 4]);
            let i32_buf = i32::from_be_bytes(_buf);
            out_buf[i] = i32_buf as f32 * factor;
        }
    }

    pub fn convert_i32_to_i16(&mut self, in_buf: &[u8], out_buf: &mut [i16]) {
        let mut _buf = [0_u8; 4];
        //assert_eq!(out_buf.len(), in_buf.len());
        for i in 0..in_buf.len() / 4 {
            _buf.copy_from_slice(&in_buf[4 * i..4 * i + 4]);
            let i32_buf = i32::from_be_bytes(_buf);
            out_buf[i] = (i32_buf >> 9) as i16;
        }
    }

    pub fn convert_i16_to_f(&mut self, in_buf: &[u8], out_buf: &mut [f32]) {
        let mut _buf = [0_u8; 2];
        //assert_eq!(out_buf.len(), in_buf.len());
        let factor = 1.0_f32 / i16::MAX as f32;
        let itr = (0..in_buf.len() / 2).rev();
        for i in itr {
            _buf.copy_from_slice(&in_buf[2 * i..2 * i + 2]);
            let i16_buf = i16::from_be_bytes(_buf);
            out_buf[i] = i16_buf as f32 * factor;
        }
    }

    pub fn convert_i16_to_i32(&mut self, in_buf: &[u8], out_buf: &mut [i32]) {
        let mut _buf = [0_u8; 2];
        //assert_eq!(out_buf.len(), in_buf.len());
        let itr = (0..in_buf.len() / 2).rev();
        for i in itr {
            _buf.copy_from_slice(&in_buf[2 * i..2 * i + 2]);
            let i16_buf = i16::from_be_bytes(_buf);
            out_buf[i] = (i16_buf as i32) << 9;
        }
    }

    pub fn convert_i8_f(&mut self, in_buf: &[u8], out_buf: &mut [f32]) {
        //assert_eq!(out_buf.len(), in_buf.len());
        let factor = 1.0_f32 / i8::MAX as f32;
        let itr = (0..in_buf.len()).rev();
        for i in itr {
            out_buf[i] = i8::from_be_bytes(*std::array::from_ref(&in_buf[i])) as f32 * factor;
        }
    }

    pub fn convert_i8_to_i32(&mut self, in_buf: &[u8], out_buf: &mut [i32]) {
        //assert_eq!(out_buf.len(), in_buf.len());
        let itr = (0..in_buf.len()).rev();
        for i in itr {
            out_buf[i] = (i8::from_be_bytes(*std::array::from_ref(&in_buf[i])) as i32) << 17;
        }
    }

    pub fn convert_f_to_f(&mut self, in_buf: &[u8], out_buf: &mut [f32]) {
        let mut _buf = [0_u8; 4];
        for i in 0..in_buf.len() / 4 {
            _buf.copy_from_slice(&in_buf[4 * i..4 * i + 4]);
            let f32_buf = f32::from_be_bytes(_buf);
            out_buf[i] = f32_buf;
        }
    }
}
