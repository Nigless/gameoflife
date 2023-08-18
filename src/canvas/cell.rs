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
const FOV: usize = 8;
const INPUTS: usize = FOV * 3 + 1;
const OUTPUTS: usize = FOV + 4;
const LAYERS: usize = 0;
const WEIGHTS: usize = INPUTS * OUTPUTS;

pub struct Cell {
    died: bool,
    energy: f32,
    weights: Vec<f32>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            died: false,
            energy: 100.0,
            weights: Vec::new(),
        }
    }

    pub fn update(&mut self, input: Vec<f32>, output_count: usize) -> Vec<f32> {
        let mut output: Vec<f32> = Vec::with_capacity(output_count);
        let mut rng = thread_rng();

        for i in 0..input.len() {
            for j in 0..output_count {
                let pos = (i + 1 * j + 1) - 1;

                if self.weights.get(pos).is_none() {
                    self.weights.insert(pos, rng.gen_range(0.0..1.0))
                }

                output.push((input[i] * self.weights[pos]) / input.len() as f32)
            }
        }

        output
    }

    pub fn die(&mut self) {
        self.died = true
    }

    pub fn eat(&mut self, cell: Cell) {
        self.energy += cell.energy
    }

    pub fn divide(&mut self) -> Self {
        let mut rng = thread_rng();
        let mut weights = self.weights.clone();
        for i in 0..weights.len() {
            if rng.gen_bool(0.4) {
                weights[i] += rng.gen_range(-0.1..0.1)
            }
        }

        Self {
            died: false,
            energy: self.energy / 2.0,
            weights,
        }
    }

    pub fn get_values(&self) -> [f32; 3] {
        [self.get_energy(), self.get_energy(), self.get_dna()]
    }

    fn get_died(&self) -> f32 {
        if self.died {
            0.0
        } else {
            1.0
        }
    }

    pub fn get_energy(&self) -> f32 {
        self.energy
    }

    pub fn get_dna(&self) -> f32 {
        let mut i: i32 = -1;
        let list_sum: f32 = self
            .weights
            .iter()
            .map(|v| {
                i += 1;
                v * fPos(i as u32)
            })
            .sum();

        let mut weight_sum = 0.0;
        for i in 0..self.weights.len() {
            weight_sum += fPos(i as u32)
        }

        list_sum / weight_sum
    }
}
