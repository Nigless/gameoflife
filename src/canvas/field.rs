use std::collections::HashMap;

use super::cell::Cell;
use crate::lib::enum_length::EnumLength;
use derives::EnumLength;

/// field unit
type Fu = u16;

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

    fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(c) = &mut self.data[(x * y) as usize] {
                    let (rx, ry, action) =
                        c.update(|rx, ry| self.get_cell(rx + x as isize, ry + y as isize));

                    let target = self.get_cell(rx + x as isize, ry + y as isize);

                    match action {
                        Action::Die => c.die(),
                        Action::Divide => {
                            if target.is_some() {
                                continue;
                            }

                            self.set_cell(rx + x as isize, ry + y as isize, Some(c.divide()))
                        }
                        Action::Eat => {
                            if target.is_none() {
                                continue;
                            }

                            c.eat(target.unwrap());

                            self.set_cell(x as isize, y as isize, None);
                            self.set_cell(rx + x as isize, ry + y as isize, Some(*c))
                        }
                        Action::Move => {
                            if target.is_some() {
                                continue;
                            }

                            self.set_cell(x as isize, y as isize, None);
                            self.set_cell(rx + x as isize, ry + y as isize, Some(*c))
                        }
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
}
