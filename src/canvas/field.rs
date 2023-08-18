use std::collections::{hash_map, HashMap};

use super::cell::Cell;
use crate::lib::enum_length::EnumLength;
use derives::EnumLength;

/// field unit
type Fu = u32;

#[derive(EnumLength)]
pub enum Action {
    Die,
    Move,
    Eat,
    Divide,
}

pub struct Field {
    width: Fu,
    height: Fu,
    data: HashMap<(Fu, Fu), Cell>,
}

impl Field {
    pub fn new(width: Fu, height: Fu) -> Self {
        let mut data = HashMap::with_capacity((width * height) as usize);

        for x in 0..width {
            for y in 0..height {
                data.insert((x, y), Cell::new());
            }
        }

        Self {
            width,
            height,
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
                Some(c) => c.get_values(),
                None => [0.0, 0.0, 0.0],
            };

            result.push(values[0]);
            result.push(values[1]);
            result.push((cell.get_dna() - values[2]).abs());
        }
        result.push(cell.get_energy());

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
            0 => Action::Die,
            1 => Action::Divide,
            2 => Action::Eat,
            3 => Action::Move,
            _ => panic!(""),
        };

        (x, y, action)
    }

    pub fn update(&mut self) {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                let pos = (x as Fu, y as Fu);

                if self.data.get(&pos).is_none() {
                    continue;
                }

                self.data.get(&pos).as_mut();

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
                    self.data.get(&pos).as_ref().unwrap(),
                );

                let output = self
                    .data
                    .get_mut(&pos)
                    .unwrap()
                    .update(input, Action::LENGTH + 4);

                let (rx, ry, action) = self.values_to_action(output);

                let target_x = rx + x;
                let target_y = ry + y as isize;
                let target = self.get_cell(target_x, target_y);

                match action {
                    Action::Die => self.data.get_mut(&pos).unwrap().die(),
                    Action::Divide => {
                        if target.is_some() {
                            continue;
                        }

                        let new_cell = self.data.get_mut(&pos).unwrap().divide();
                        self.set_cell(target_x, target_y, new_cell)
                    }
                    Action::Eat => {
                        let target = self.remove_cell(target_x, target_y);

                        let mut cell = self.remove_cell(x, y).unwrap();

                        if let Some(target) = target {
                            cell.eat(target);
                        }

                        self.set_cell(target_x, target_y, cell);
                    }
                    Action::Move => {
                        if target.is_some() {
                            continue;
                        }

                        let cell = self.remove_cell(x, y).unwrap();

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

    pub fn get_data(&self) -> &HashMap<(Fu, Fu), Cell> {
        &self.data
    }
}
