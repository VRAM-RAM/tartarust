use std::ops::{Index, IndexMut};
use std::convert::TryFrom;
use crate::rounds::quarter_round::{QuarterState};

#[derive(Debug)]
pub struct State<'a>(&'a mut [u32; 16]);

impl<'a> Index<usize> for State<'a> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IndexMut<usize> for State<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a> TryFrom<&'a mut [u32]> for State<'a> {
    type Error = &'static str;

    fn try_from(slice: &'a mut [u32]) -> Result<Self, Self::Error> {
        let words: &'a mut [u32; 16] = slice
            .try_into()
            .map_err(|_| "State must contain exactly 16 words")?;

        Ok(Self(words))
    }
}

impl<'a> State<'a> {
    pub fn new(array : &'a mut [u32; 16]) -> Self {
        Self(array)
    }

    pub fn round(&mut self) {
        // Column rounds
        self.apply_quarter_round((0, 4, 8, 12));
        self.apply_quarter_round((1, 5, 9, 13));

        self.apply_quarter_round((2, 6, 10, 14));
        self.apply_quarter_round((3, 7, 11, 15));

        // Diagonal rounds
        self.apply_quarter_round((0, 5, 10, 15));
        self.apply_quarter_round((1, 6, 11, 12));

        self.apply_quarter_round((2, 7, 8, 13));
        self.apply_quarter_round((3, 4, 9, 14));
    }

    fn apply_quarter_round(&mut self, positions: (usize, usize, usize, usize)) {
        let (a, b, c, d) = positions;
        let mut quarter = QuarterState::new(self[a], self[b], self[c], self[d]);
        quarter.round();

        let [a2, b2, c2, d2] = quarter.into_words();

        self[a] = a2;
        self[b] = b2;
        self[c] = c2;
        self[d] = d2;
    }
}

