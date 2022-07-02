use crate::decoder::DecoderTrait;

pub struct Pcmf32<R> {
    reader: R
}

impl<R> DecoderTrait<R> for Pcmf32<R> {
    fn new(reader: R) -> Self {
        todo!()
    }

    fn into_inner(s: Self) -> R
    where Self: Sized {
        todo!()
    }
}