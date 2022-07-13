pub fn convert_f_to_i32(in_buf: &[f32], out_buf: &mut [i32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    for i in 0..in_buf.len() {
        out_buf[i] = (in_buf[i] * (1 << 24) as f32) as i32;
    }
}

pub fn convert_f_to_i16(in_buf: &[f32], out_buf: &mut [i16]) {
    assert_eq!(out_buf.len(), in_buf.len());
    for i in 0..in_buf.len() {
        out_buf[i] = (in_buf[i] * i16::MAX as f32) as i16;
    }
}

pub fn convert_i32_to_f(in_buf: &[i32], out_buf: &mut [f32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    let factor = 1.0_f32 / (1 << 24) as f32;
    for i in 0..in_buf.len() {
        out_buf[i] = in_buf[i] as f32 * factor;
    }
}

pub fn convert_i32_to_i16(in_buf: &[i32], out_buf: &mut [i16]) {
    assert_eq!(out_buf.len(), in_buf.len());
    for i in 0..in_buf.len() {
        out_buf[i] = (in_buf[i] >> 9) as i16;
    }
}

pub fn convert_i16_to_f(in_buf: &[i16], out_buf: &mut [f32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    let factor = 1.0_f32 / i16::MAX as f32;
    let itr = (0..in_buf.len()).rev();
    for i in itr {
        out_buf[i] = in_buf[i] as f32 * factor;
    }
}

pub fn convert_i16_to_i32(in_buf: &[i16], out_buf: &mut [i32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    let itr = (0..in_buf.len()).rev();
    for i in itr {
        out_buf[i] = (in_buf[i] as i32) << 9;
    }
}

pub fn convert_i8_f(in_buf: &[i8], out_buf: &mut [f32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    let factor = 1.0_f32 / i8::MAX as f32;
    let itr = (0..in_buf.len()).rev();
    for i in itr {
        out_buf[i] = in_buf[i] as f32 * factor;
    }
}

pub fn convert_i8_to_i32(in_buf: &[i8], out_buf: &mut [i32]) {
    assert_eq!(out_buf.len(), in_buf.len());
    let itr = (0..in_buf.len()).rev();
    for i in itr {
        out_buf[i] = (in_buf[i] as i32) << 17;
    }
}

pub fn convert_f_to_f(in_buf: &[f32], out_buf: &mut [f32]) {
    out_buf.copy_from_slice(in_buf);
}