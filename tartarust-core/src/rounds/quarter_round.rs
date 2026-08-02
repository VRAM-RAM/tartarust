
#[derive(Clone, Copy)]
pub struct QuarterState {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl QuarterState {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self { a, b, c, d }
    }

    pub fn into_words(self) -> [u32; 4] {
        [self.a, self.b, self.c, self.d]
    }

    #[inline]
    pub fn round(&mut self) {
        self.a = self.a.wrapping_add(self.b);
        self.d ^= self.a;
        self.d = self.d.rotate_left(16);

        self.c = self.c.wrapping_add(self.d);
        self.b ^= self.c;
        self.b = self.b.rotate_left(12);

        self.a = self.a.wrapping_add(self.b);
        self.d ^= self.a;
        self.d = self.d.rotate_left(8);

        self.c = self.c.wrapping_add(self.d);
        self.b ^= self.c;
        self.b = self.b.rotate_left(7);
    }
}




















