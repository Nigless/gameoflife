use crate::lib::enum_length::EnumLength;

use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::field::Action;

fn fPos(pos: u32) -> f32 {
    pos as f32 * 0.1 + 1.0
}
///```
/// x  x  x
/// x  o  x
/// x  x  x
/// ```
const INPUTS: usize = 8 * 3 + 1;
const OUTPUTS: usize = 8 + 4;
const LAYERS: usize = 0;
const WEIGHTS: usize = INPUTS * OUTPUTS;

pub struct Cell {
    died: bool,
    energy: f32,
    weights: [f32; WEIGHTS],
}

impl Cell {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let mut weights = [(); WEIGHTS].map(|_| rng.gen_range(0.0..1.0) as f32);

        Self {
            died: false,
            energy: 100.0,
            weights,
        }
    }

    pub fn update<'a, G: Fn(isize, isize) -> &'a Option<Cell>>(
        &self,
        get_cell: G,
    ) -> (isize, isize, Action) {
        let mut inputs = [0.0 as f32; INPUTS];
        let cells = [
            get_cell(-1, 1),
            get_cell(0, 1),
            get_cell(1, 1),
            get_cell(-1, 0),
            get_cell(1, 0),
            get_cell(-1, -1),
            get_cell(0, -1),
            get_cell(1, -1),
        ];

        let inp = inputs[0];

        for i in 0..cells.len() {
            let j = i * 3;

            if let Some(c) = cells[i] {
                inputs[j] = c.get_died();
                inputs[j + 1] = c.get_energy();
                inputs[j + 2] = (self.get_dna() - c.get_dna()).abs()
            }
        }
        inputs[INPUTS - 1] = self.get_energy();

        let mut output = [0.0; OUTPUTS];

        for i in 0..inputs.len() {
            for j in 0..output.len() {
                output[j] = output[j] + (inputs[i] * self.weights[i * j]) / INPUTS as f32
            }
        }

        let mut pos = 0;
        let mut value = 0.0;

        for i in 0..output[0..8].len() {
            if output[i] > value {
                pos = i;
                value = output[i];
            }
        }

        let (x, y) = match pos {
            0 => (-1, 1),
            1 => (0, 1),
            2 => (1, 1),
            3 => (-1, 0),
            4 => (1, 0),
            5 => (-1, -1),
            6 => (0, -1),
            7 => (1, -1),
        };

        let mut pos = 0;
        let mut value = 0.0;

        for i in 0..output[8..OUTPUTS].len() {
            if output[i] > value {
                pos = i;
                value = output[i];
            }
        }

        let action = match pos + 8 {
            0 => Action::Die,
            1 => Action::Divide,
            2 => Action::Eat,
            3 => Action::Move,
        };

        (x, y, action)
    }

    pub fn die(&mut self) {
        self.died = true
    }

    pub fn eat(&mut self, cell: Cell) {
        self.energy += cell.energy
    }

    pub fn divide(&mut self) -> Self {
        let mut rng = thread_rng();
        let mut weights = self.weights;
        for i in 0..self.weights.len() {
            if rng.gen_bool(0.4) {
                weights[i] = self.weights[i] + rng.gen_range(-0.1..0.1)
            }
        }

        Self {
            died: false,
            energy: self.energy / 2.0,
            weights,
        }
    }

    fn get_died(&self) -> f32 {
        if self.died {
            0.0
        } else {
            1.0
        }
    }

    fn get_energy(&self) -> f32 {
        self.energy
    }

    fn get_dna(&self) -> f32 {
        let mut i: i32 = -1;
        let list_sum: f32 = self
            .weights
            .map(|v| {
                i += 1;
                v * fPos(i as u32)
            })
            .iter()
            .sum();

        let mut weight_sum = 0.0;
        for i in 0..self.weights.len() {
            weight_sum += fPos(i as u32)
        }

        list_sum / weight_sum
    }
}
