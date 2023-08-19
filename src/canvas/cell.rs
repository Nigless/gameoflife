use std::cmp::{max, min};

use crate::lib::enum_length::EnumLength;

use rand::{rngs::ThreadRng, thread_rng, Rng};

fn f_pos(pos: u32) -> f32 {
    pos as f32 * 0.1 + 1.0
}

pub struct Cell {
    pub died: bool,
    pub energy: u16,
    weights: Vec<f32>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            died: false,
            energy: 100,
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

    pub fn lose_enegry(&mut self, amount: u16) {
        if self.energy <= amount {
            self.energy = 0;
            self.die()
        } else {
            self.energy -= amount
        }
    }

    pub fn take_energy(&mut self, amount: u16) -> u16 {
        if self.energy <= amount {
            self.die();
            let result = self.energy;
            self.energy = 0;
            result
        } else {
            self.energy -= amount;
            amount
        }
    }

    pub fn give_energy(&mut self, amount: u16) {
        if self.energy >= 100 {
            return;
        }

        self.energy += min(100 - self.energy, amount);
    }

    pub fn die(&mut self) {
        self.died = true
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
            energy: self.energy / 2,
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
        self.energy as f32 / 100.0
    }

    pub fn get_dna(&self) -> f32 {
        let mut i: i32 = -1;
        let list_sum: f32 = self
            .weights
            .iter()
            .map(|v| {
                i += 1;
                v * f_pos(i as u32)
            })
            .sum();

        let mut weight_sum = 0.0;
        for i in 0..self.weights.len() {
            weight_sum += f_pos(i as u32)
        }

        list_sum / weight_sum
    }
}
