use crate::decoder::DecoderTrait;

pub struct Pcmi8<R> {
    reader: R
}

impl<R> DecoderTrait<R> for Pcmi8<R> {
    fn new(reader: R) -> Self {
        todo!()
    }

    fn into_inner(s: Self) -> R
    where Self: Sized {
        todo!()
    }
}