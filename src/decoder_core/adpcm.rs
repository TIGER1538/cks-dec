use crate::{
    decoder_core::core::DecoderCore,
    error::{AdpcmError, CksError},
};
use std::io::{Read, Seek};

#[derive(Debug)]
pub(crate) struct AdpcmCore {
    bytes_per_block: usize,
    buf: Vec<u8>,
}

static BYTES_PER_BLOCK_DEFAULT: usize = 24;
static FIXED_POINT_COEF_BASE: i32 = 256;
static FIXED_POINT_ADAPTION_BASE: i32 = 256;
static CK_INT16_MIN: i16 = (-0x7FFF - 1) as i16;
static CK_INT16_MAX: i16 = i16::MAX;
static MIN_DELTA: i16 = 16;
static COEFFS: [[i32; 2]; 7] = [
    [256, 0],
    [512, -256],
    [0, 0],
    [192, 64],
    [240, 0],
    [460, -208],
    [392, -232],
];
static ADAPTION_TABLE: [i16; 16] = [
    230, 230, 230, 230, 307, 409, 512, 614, 768, 614, 512, 409, 307, 230, 230, 230,
];

impl AdpcmCore {
    pub fn decode<R: Read + Seek>(decoder_core: &mut DecoderCore<R>, output_buf: &mut [i16]) -> Option<usize> {
        let mut decoded_bytes = 0;
        if let None = decoder_core.adpcm_core {
            decoder_core.adpcm_core = Some(Self {
                bytes_per_block: BYTES_PER_BLOCK_DEFAULT,
                buf: Vec::with_capacity(BYTES_PER_BLOCK_DEFAULT * 2),
            });
        }
        let block_read = Self::read(decoder_core, 1);
        assert!(block_read == Some(1) || block_read == None);

        let buf_read = &mut decoder_core.adpcm_core.as_mut().unwrap().buf;
        if block_read == Some(0) {
            return None;
        } else {
            decoded_bytes += Self::dec_core(&buf_read[..], BYTES_PER_BLOCK_DEFAULT, output_buf, decoder_core.sample_info.channels).unwrap();
            if decoder_core.sample_info.channels == 2 {
                decoded_bytes += Self::dec_core(&buf_read[BYTES_PER_BLOCK_DEFAULT..], BYTES_PER_BLOCK_DEFAULT, output_buf, decoder_core.sample_info.channels).unwrap();
            }
        }
        Some(decoded_bytes)
    }

    fn dec_core(
        in_buf: &[u8],
        input_byte: usize,
        out_buf: &mut [i16],
        output_stride: u8,
    ) -> Result<usize, AdpcmError> {
        if !(output_stride == 1 || output_stride == 2) {
            eprintln!("output stride = {}", output_stride);
            return Err(AdpcmError::InvalidStride);
        } else if input_byte < 7 {
            eprintln!("input bytes = {}", input_byte);
            return Err(AdpcmError::NoEnoughInputBytes);
        }
        let output_stride = output_stride as usize;

        let input_end = input_byte;
        let predictor = in_buf[0] as usize;
        let mut delta = i16::from_be_bytes(in_buf[1..3].try_into().unwrap());
        let (mut samp2, mut samp1) = (
            i16::from_be_bytes(in_buf[3..5].try_into().unwrap()),
            i16::from_be_bytes(in_buf[5..7].try_into().unwrap()),
        );

        let (mut input_index, mut output_index) = (7_usize, 0);
        out_buf[output_index] = samp2;
        output_index += output_stride;
        out_buf[output_index] = samp1;
        output_index += output_stride;

        let coef1 = COEFFS[predictor][0];
        let coef2 = COEFFS[predictor][1];

        while output_index < input_end {
            for nybble in 0..2 {
                let mut pred_samp =
                    ((samp1 as i32 * coef1) + (samp2 as i32 * coef2)) / FIXED_POINT_COEF_BASE;
                let error_delta = (in_buf[input_index] >> (nybble * 4)) & 0xF;
                if (error_delta & 0x8) != 0 {
                    pred_samp += delta as i32 * (error_delta as i32 - 0x10);
                } else {
                    pred_samp += delta as i32 * error_delta as i32;
                }
                let new_samp = clamp(pred_samp, CK_INT16_MIN as i32, CK_INT16_MAX as i32);
                out_buf[output_index] = new_samp as i16;
                output_index += output_stride;

                //println!("{}, {}", delta, ADAPTION_TABLE[error_delta as usize]);
                delta = ((delta as isize * ADAPTION_TABLE[error_delta as usize] as isize) as i16 / FIXED_POINT_ADAPTION_BASE as i16) as i16;
                if delta < MIN_DELTA {
                    delta = MIN_DELTA;
                }

                samp2 = samp1;
                samp1 = new_samp as i16;
            }
            input_index += 1;
        }

        /*
        for content_idx in input_index..input_end {
            //let mut out_byte = 0_u8;
            for nybble in 0..2 {
                let mut pred_samp =
                    ((samp1 as i32 * coef1) + (samp2 as i32 * coef2)) / FIXED_POINT_COEF_BASE;
                let error_delta = (in_buf[content_idx] >> (nybble * 4)) & 0xF;
                if (error_delta & 0x8) != 0 {
                    pred_samp += delta as i32 * (error_delta as i32 - 0x10);
                } else {
                    pred_samp += delta as i32 * error_delta as i32;
                }
                let new_samp = clamp(pred_samp, CK_INT16_MIN as i32, CK_INT16_MAX as i32);
                out_buf[output_index] = new_samp as i16;
                output_index += output_stride;

                delta = (delta * ADAPTION_TABLE[error_delta as usize] / FIXED_POINT_ADAPTION_BASE as i16) as i16;
                if delta < MIN_DELTA {
                    delta = MIN_DELTA;
                }

                samp2 = samp1;
                samp1 = new_samp as i16;
            }
        }
        */

        let output_samples = (out_buf.len()) / output_stride;
        Ok(output_samples)
    }

    fn read<R: Read + Seek>(decoder_core: &mut DecoderCore<R>, blocks: usize) -> Option<u64> {
        let core = decoder_core.adpcm_core.as_mut().unwrap();
        let bytes = core.buf.len();
        let buf = &mut core.buf;
        if bytes != BYTES_PER_BLOCK_DEFAULT * 2 {
            buf.resize(BYTES_PER_BLOCK_DEFAULT * 2, 0);
        }
        let bytes_to_end = std::cmp::max(
            decoder_core.stream_size - decoder_core.reader.stream_position().unwrap(),
            0,
        );
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end) as usize;
        if bytes_to_read > 0 {
            if buf.len() < bytes_to_read {
                buf.resize(bytes_to_read, 0);
            }
            decoder_core
                .reader
                .read(&mut buf[0..bytes_to_read])
                .unwrap();
        } else {
            return None;
        }
        println!("{:?}", buf);
        Some((bytes_to_read / (BYTES_PER_BLOCK_DEFAULT * 2)) as _)
    }
}

#[inline]
fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
