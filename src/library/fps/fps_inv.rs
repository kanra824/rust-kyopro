use crate::library::fps::fps::Fps;

pub trait FpsInv {
    fn inv(&self, n: usize) -> Self;
}

impl FpsInv for Fps {
}