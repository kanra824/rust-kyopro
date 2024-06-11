use std::time::{Instant, SystemTime};

#[derive(Debug)]
struct XorShift {
    w: u32,
    x: u32,
    y: u32,
    z: u32,
}

impl XorShift {
    fn new() -> Self {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let seed = d.as_secs() as u32;
        Self::from_seed(seed)
    }

    fn from_seed(seed: u32) -> Self {
        let w = seed;
        let x = w << 13;
        let y = (w >> 9) ^ (x << 6);
        let z = y >> 7;
        Self { w, x, y, z }
    }

    fn rand(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19) ^ (t ^ (t >> 8)));
        self.w
    }

    // [min, max] のu32乱数
    fn rand_u32(&mut self, min: u32, max: u32) -> u32 {
        self.rand() % (max - min + 1) + min
    }

    // [min, max] のf64乱数
    fn rand_double(&mut self, min: f64, max: f64) -> f64 {
        (self.rand() % 0xFFFF) as f64 / (0xFFFF as f64 * (max - min) + min)
    }
}
