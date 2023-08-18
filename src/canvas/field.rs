use std::collections::HashMap;

use super::cell::Cell;
use crate::lib::enum_length::EnumLength;
use derives::EnumLength;

/// field unit
type Fu = u16;

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
    data: Vec<Option<Cell>>,
}

impl Field {
    pub fn new(width: Fu, height: Fu) -> Self {
        let mut data = Vec::<Option<Cell>>::with_capacity((width * height) as usize);

        for i in 0..width * height {
            data[i as usize] = Some(Cell::new())
        }

        Self {
            width,
            height,
            data: data,
        }
    }

    pub fn get_width(&self) -> Fu {
        self.width
    }

    pub fn get_height(&self) -> Fu {
        self.height
    }

    fn cells_to_values(&self, arr: &[&Option<Cell>], cell: &Cell) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::new();
        for i in 0..arr.len() {
            let j = i * 3;

            let values = match &arr[i] {
                Some(c) => c.get_values(),
                None => [0.0, 0.0, 0.0],
            };

            result[j] = values[0];
            result[j + 1] = values[1];
            result[j + 2] = (cell.get_dna() - values[2]).abs()
        }
        result.push(cell.get_energy());

        result
    }

    fn update(&mut self) {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                let index = (x * y) as usize;

                if self.data[index].is_none() {
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
                    self.data[index].as_ref().unwrap(),
                );

                let output = self.data[index]
                    .as_mut()
                    .unwrap()
                    .update(input, Action::LENGTH + 4);

                let target_x = rx + x;
                let target_y = ry + y as isize;
                let target = self.get_cell(target_x, target_y);

                match action {
                    Action::Die => self.data[index].as_mut().unwrap().die(),
                    Action::Divide => {
                        if target.is_some() {
                            continue;
                        }

                        let new_cell = self.data[index].as_mut().unwrap().divide();
                        self.set_cell(target_x, target_y, Some(new_cell))
                    }
                    Action::Eat => {
                        let target = self.remove_cell(target_x, target_y);

                        let cell = self.remove_cell(x, y);

                        self.set_cell(target_x, target_y, cell);

                        if let Some(target) = target {
                            self.data[index].as_mut().unwrap().eat(target);
                        }
                    }
                    Action::Move => {
                        if target.is_some() {
                            continue;
                        }

                        let cell = self.remove_cell(x, y);

                        self.set_cell(target_x, target_y, cell);
                    }
                }
            }
        }
    }

    fn map_x(&self, mut x: isize) -> usize {
        if x >= 0 && x < self.width as isize {
            return x as usize;
        }

        x = x % self.width as isize;

        if x < 0 {
            x = self.width as isize - x;
        }

        x as usize
    }

    fn map_y(&self, mut y: isize) -> usize {
        if y >= 0 && y < self.height as isize {
            return y as usize;
        }

        y = y % self.height as isize;

        if y < 0 {
            y = self.height as isize - y;
        }

        y as usize
    }

    fn set_cell(&mut self, x: isize, y: isize, cell: Option<Cell>) {
        let pos = self.map_x(x) * self.map_y(y);
        self.data[pos] = cell
    }

    fn get_cell(&self, x: isize, y: isize) -> &Option<Cell> {
        &self.data[self.map_x(x) * self.map_y(y)]
    }

    fn remove_cell(&mut self, x: isize, y: isize) -> Option<Cell> {
        let pos = self.map_x(x) * self.map_y(y);

        if self.data[pos].is_some() {
            let cell = self.data.remove(pos);
            self.data.insert(pos, None);
            return cell;
        }
        None
    }
}
