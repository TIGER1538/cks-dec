use crate::{decoder_core::core::DecoderCore, error::AdpcmError};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub(crate) struct AdpcmCore {
    bytes_per_block: usize,
    buf: Vec<u8>,
}

pub(crate) static BYTES_PER_BLOCK_DEFAULT: usize = 24;
static FIXED_POINT_COEF_BASE: i32 = 256;
static FIXED_POINT_ADAPTION_BASE: i32 = 256;
static CK_INT16_MIN: i16 = (-0x7FFF - 1) as i16;
static CK_INT16_MAX: i16 = 0x7FFF;
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

//static mut TEST_I: usize = 0;

impl AdpcmCore {
    pub fn decode<R: Read + Seek>(
        decoder_core: &mut DecoderCore<R>,
        output_buf: &mut [i16],
    ) -> Option<usize> {
        let mut decoded_bytes = 0;
        if decoder_core.adpcm_core.is_none() {
            decoder_core.adpcm_core = Some(Self {
                bytes_per_block: BYTES_PER_BLOCK_DEFAULT,
                buf: Vec::with_capacity(BYTES_PER_BLOCK_DEFAULT * 2),
            });
        }
        let block_read = Self::read(decoder_core, 1);
        assert!(block_read == Some(1) || block_read == None);

        let buf_read = &mut decoder_core.adpcm_core.as_mut().unwrap().buf;
        if block_read == None {
            return None;
        } else {
            decoded_bytes += Self::dec_core(
                &buf_read[..BYTES_PER_BLOCK_DEFAULT],
                BYTES_PER_BLOCK_DEFAULT,
                output_buf,
                decoder_core.sample_info.channels,
            )
            .unwrap();
            if decoder_core.sample_info.channels == 2 {
                decoded_bytes += Self::dec_core(
                    &buf_read[BYTES_PER_BLOCK_DEFAULT..],
                    BYTES_PER_BLOCK_DEFAULT,
                    &mut output_buf[1..],
                    decoder_core.sample_info.channels,
                )
                .unwrap();
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
        let (mut input_index, mut output_index) = (0_usize, 0);

        let input_end = input_byte;
        let predictor = in_buf[input_index] as usize;
        input_index += 1;
        //let mut delta = i16::from_be_bytes(in_buf[input_index..input_index+2].try_into().unwrap());
        let mut delta =
            i16::from_le_bytes(in_buf[input_index..input_index + 2].try_into().unwrap());
        input_index += 2;
        let (mut samp2, mut samp1) = (
            i16::from_le_bytes(in_buf[input_index..input_index + 2].try_into().unwrap()),
            i16::from_le_bytes(in_buf[input_index + 2..input_index + 4].try_into().unwrap()),
        );
        input_index += 4;

        out_buf[output_index] = samp2;
        output_index += output_stride;
        out_buf[output_index] = samp1;
        output_index += output_stride;

        let coef1 = COEFFS[predictor][0];
        let coef2 = COEFFS[predictor][1];

        while input_index < input_end {
            for nybble in 0..2 {
                let mut pred_samp =
                    ((samp1 as i32 * coef1) + (samp2 as i32 * coef2)) / FIXED_POINT_COEF_BASE;
                let error_delta = ((in_buf[input_index] >> (nybble * 4)) & 0xF) as i32;
                if (error_delta & 0x8) != 0 {
                    pred_samp += delta as i32 * (error_delta - 0x10);
                } else {
                    pred_samp += delta as i32 * error_delta;
                }
                let new_samp = clamp(pred_samp, CK_INT16_MIN as i32, CK_INT16_MAX as i32);
                out_buf[output_index] = new_samp as i16;
                output_index += output_stride;

                //println!("{}, {}", delta, ADAPTION_TABLE[error_delta as usize]);
                delta = ((delta as isize * ADAPTION_TABLE[error_delta as usize] as isize)
                    / FIXED_POINT_ADAPTION_BASE as isize) as i16;
                if delta < MIN_DELTA {
                    delta = MIN_DELTA;
                }

                samp2 = samp1;
                samp1 = new_samp as i16;
            }
            input_index += 1;
        }

        let output_samples = output_index / output_stride;
        assert_eq!(output_samples, 2 * input_byte - 12);
        Ok(output_samples)
    }

    fn read<R: Read + Seek>(decoder_core: &mut DecoderCore<R>, blocks: usize) -> Option<u64> {
        let core = decoder_core.adpcm_core.as_mut().unwrap();
        let mut bytes = core.buf.len();
        let buf = &mut core.buf;
        if bytes != BYTES_PER_BLOCK_DEFAULT * 2 * blocks {
            buf.resize(BYTES_PER_BLOCK_DEFAULT * 2 * blocks, 0);
            bytes = buf.len();
        }
        let bytes_to_end = std::cmp::max(
            decoder_core.stream_size - decoder_core.reader.stream_position().unwrap(),
            0,
        );
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end) as usize;
        if bytes_to_read > 0 {
            'check_byte: while let Ok(res) = decoder_core.reader.read(&mut buf[0..bytes_to_read]) {
                if res != bytes_to_read {
                    decoder_core
                        .reader
                        .seek(SeekFrom::Current(-(res as i64)))
                        .unwrap();
                } else {
                    break 'check_byte;
                }
            }
        } else {
            eprintln!("NONE!");
            return None;
        }

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
