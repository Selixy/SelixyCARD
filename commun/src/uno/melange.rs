#[derive(Clone, Debug)]
pub struct Alea(u64);

impl Alea {
    pub fn nouvelle(graine: u64) -> Alea {
        Alea(graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn suivant(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn borne(&mut self, n: usize) -> usize {
        (self.suivant() % n as u64) as usize
    }

    pub fn melanger<T>(&mut self, elements: &mut [T]) {
        for i in (1..elements.len()).rev() {
            let j = self.borne(i + 1);
            elements.swap(i, j);
        }
    }
}
