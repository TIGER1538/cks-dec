use crate::decoder_core::core::DecoderCore;
use std::io::{Read, Seek};

pub(crate) struct AdpcmCore {
    bytes_per_block: usize,
    buf: Vec<u8>,
}

static BYTES_PER_BLOCK_DEFAULT: usize = 24;

impl AdpcmCore {
    fn decode<R: Read + Seek>(decoder_core: &mut DecoderCore<R>) -> Option<u64> {
        if let None = decoder_core.adpcm_core {
            decoder_core.adpcm_core = Some(Self {
                bytes_per_block: BYTES_PER_BLOCK_DEFAULT,
                buf: Vec::with_capacity(BYTES_PER_BLOCK_DEFAULT * 2),
            });
        }
        let block_read = Self::read(decoder_core, 1);
        assert!(block_read == Some(1) || block_read == Some(0));

        if block_read == Some(0) {
            None
        } else {
            Some(block_read*)
        }
    }

    fn read<R: Read + Seek>(decoder_core: &mut DecoderCore<R>, blocks: usize) -> Option<u64> {
        let core = decoder_core.adpcm_core.as_mut().unwrap();
        let buf = &mut core.buf;
        let bytes = blocks * BYTES_PER_BLOCK_DEFAULT;
        let bytes_to_end =
            std::cmp::max(decoder_core.stream_size - decoder_core.reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end) as usize;
        if bytes_to_read > 0 {
            if buf.len() < bytes_to_read {
                buf.resize(bytes_to_read, 0);
            }
            decoder_core.reader.read(&mut buf[0..bytes_to_read]).unwrap();
        } else {
            return None;
        }
        Some((bytes_to_read/BYTES_PER_BLOCK_DEFAULT) as _)
    }
}
