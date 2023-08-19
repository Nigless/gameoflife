use std::{
    collections::{hash_map, HashMap},
    ops::Sub,
};

use super::cell::Cell;
use crate::lib::enum_length::EnumLength;
use derives::EnumLength;
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, Rng};

/// field unit
type Fu = u32;

#[derive(EnumLength)]
pub enum Action {
    Energize,
    Move,
    Eat,
    Divide,
    Charge,
}

pub struct Field {
    width: Fu,
    height: Fu,
    env: Perlin,
    data: HashMap<(Fu, Fu), Cell>,
}

impl Field {
    pub const MAX_ENERGY: u16 = 100;
    pub const MUTATION_CHANCE: f32 = 0.5;
    pub const MUTATION_SCALE: f32 = 0.3;

    pub const ENERGY_LOSS: u16 = 1;

    pub const CHARGE_AMOUNT: u16 = 5;

    pub const ENERGIZE_AMOUNT: u16 = 10;
    pub const ENERGIZE_COST: u16 = 2;

    pub const DIVIDE_COST: u16 = 0;

    pub const EAT_COST: u16 = 2;

    pub const MOVE_COST: u16 = 2;

    pub fn new(width: Fu, height: Fu) -> Self {
        let mut data = HashMap::with_capacity((width * height) as usize);
        let mut rng = thread_rng();

        for x in 0..width {
            for y in 0..height {
                if rng.gen_bool(0.5) {
                    continue;
                }
                data.insert((x, y), Cell::new());
            }
        }

        let perlin = Perlin::new(1);

        Self {
            width,
            height,
            env: perlin,
            data,
        }
    }

    pub fn get_width(&self) -> Fu {
        self.width
    }

    pub fn get_height(&self) -> Fu {
        self.height
    }

    fn cells_to_values(&self, cells: &[Option<&Cell>], cell: &Cell) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::new();
        for i in 0..cells.len() {
            let values = match &cells[i] {
                Some(c) => [(cell.get_dna() - c.get_dna()).abs(), c.get_died()],
                None => [0.0, 0.0],
            };

            result.push(values[0]);
            result.push(values[1]);
        }
        result.push(1.0 - cell.get_energy());

        result
    }

    fn values_to_action(&self, values: Vec<f32>) -> (isize, isize, Action) {
        let position_values = &values[Action::LENGTH..Action::LENGTH + 4];
        let action_values = &values[0..Action::LENGTH];

        let mut pos = 0;
        let mut value = 0.0;

        for i in 0..position_values.len() {
            if values[i] > value {
                pos = i;
                value = values[i];
            }
        }

        let (x, y) = match pos {
            0 => (0, 1),
            1 => (-1, 0),
            2 => (1, 0),
            3 => (0, -1),
            _ => panic!(""),
        };

        let mut pos = 0;
        let mut value = 0.0;

        for i in 0..action_values.len() {
            if values[i] > value {
                pos = i;
                value = values[i];
            }
        }

        let action = match pos {
            0 => Action::Energize,
            1 => Action::Divide,
            2 => Action::Eat,
            3 => Action::Move,
            4 => Action::Charge,
            _ => panic!(""),
        };

        (x, y, action)
    }

    pub fn update(&mut self) {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                let cur_key = (x as Fu, y as Fu);

                let cell = self.data.get_mut(&cur_key);

                if cell.is_none() {
                    continue;
                }

                let cell = cell.unwrap();

                if cell.died {
                    continue;
                }

                cell.take_energy(Self::ENERGY_LOSS);

                if cell.energy <= 0 {
                    cell.die();
                    continue;
                }

                let input = self.cells_to_values(
                    &[
                        self.get_cell(x - 1, y + 1),
                        self.get_cell(x + 0, y + 1),
                        self.get_cell(x + 1, y + 1),
                        self.get_cell(x - 1, y + 0),
                        self.get_cell(x + 1, y + 0),
                        self.get_cell(x - 1, y - 1),
                        self.get_cell(x + 0, y - 1),
                        self.get_cell(x + 1, y - 1),
                    ],
                    self.data.get(&cur_key).as_ref().unwrap(),
                );

                let output = self
                    .data
                    .get_mut(&cur_key)
                    .unwrap()
                    .update(input, Action::LENGTH + 4);

                let (rx, ry, action) = self.values_to_action(output);

                let target_x = rx + x;
                let target_y = ry + y;
                let target = self.get_cell(target_x, target_y);
                let env = self.get_env_value(x, y);

                match action {
                    Action::Charge => self
                        .data
                        .get_mut(&cur_key)
                        .unwrap()
                        .give_energy(Self::CHARGE_AMOUNT),
                    Action::Energize => {
                        if target.is_none() {
                            continue;
                        }

                        if target.unwrap().died {
                            continue;
                        }

                        self.data
                            .get_mut(&cur_key)
                            .unwrap()
                            .take_energy(Self::ENERGIZE_COST);
                        let amount = self
                            .data
                            .get_mut(&cur_key)
                            .unwrap()
                            .take_energy(Self::ENERGIZE_AMOUNT);
                        let target = self
                            .data
                            .get_mut(&(self.map_x(target_x), self.map_y(target_y)))
                            .unwrap();
                        target.give_energy(amount);
                    }
                    Action::Divide => {
                        if target.is_some() && !target.unwrap().died {
                            continue;
                        }

                        self.data
                            .get_mut(&cur_key)
                            .unwrap()
                            .take_energy(Self::DIVIDE_COST);

                        let new_cell = self.data.get_mut(&cur_key).unwrap().divide();
                        self.set_cell(target_x, target_y, new_cell)
                    }
                    Action::Eat => {
                        let target = self.remove_cell(target_x, target_y);

                        let mut cell = self.remove_cell(x, y).unwrap();

                        if let Some(target) = target {
                            cell.give_energy(target.energy);
                        }

                        cell.take_energy(Self::EAT_COST);

                        self.set_cell(target_x, target_y, cell);
                    }
                    Action::Move => {
                        if target.is_some() && !target.unwrap().died {
                            continue;
                        }

                        let mut cell = self.remove_cell(x, y).unwrap();
                        EAT
                        cell.take_energy(Self::MOVE_COST);

                        self.set_cell(target_x, target_y, cell);
                    }
                }
            }
        }
    }

    fn map_x(&self, mut x: isize) -> Fu {
        if x >= 0 && x < self.width as isize {
            return x as Fu;
        }

        x = x % self.width as isize;

        if x < 0 {
            x = self.width as isize - x;
        }

        x as Fu
    }

    fn map_y(&self, mut y: isize) -> Fu {
        if y >= 0 && y < self.height as isize {
            return y as Fu;
        }

        y = y % self.height as isize;

        if y < 0 {
            y = self.height as isize - y;
        }

        y as Fu
    }

    fn set_cell(&mut self, x: isize, y: isize, cell: Cell) {
        self.data.insert((self.map_x(x), self.map_y(y)), cell);
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<&Cell> {
        self.data.get(&(self.map_x(x), self.map_y(y)))
    }

    fn remove_cell(&mut self, x: isize, y: isize) -> Option<Cell> {
        self.data.remove(&(self.map_x(x), self.map_y(y)))
    }

    fn get_env_value(&self, x: isize, y: isize) -> f32 {
        self.env.get([self.map_x(x) as f64, self.map_x(y) as f64]) as f32
    }

    pub fn get_data(&self) -> &HashMap<(Fu, Fu), Cell> {
        &self.data
    }
}
