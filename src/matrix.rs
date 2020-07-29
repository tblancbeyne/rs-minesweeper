use std::collections::{HashSet, VecDeque};

use sfml::graphics::{
    Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text, Transformable,
};

use sfml::system::Vector2f;

#[derive(Debug, Eq, PartialEq)]
pub enum Cell {
    Bomb,
    Empty(usize),
}

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<(Cell, bool, bool)>>,
    scale: f32,
}

impl Matrix {
    pub fn new(size: (usize, usize, usize), scale: f32) -> Self {
        let mut bombs = HashSet::new();

        while bombs.len() < size.2 {
            let x = rand::random::<usize>() % size.0;
            let y = rand::random::<usize>() % size.1;
            bombs.insert((x, y));
        }

        let mut data = Vec::new();

        for _i in 0..size.0 {
            let mut row = Vec::new();
            for _j in 0..size.1 {
                row.push((Cell::Empty(0), false, false));
            }
            data.push(row);
        }

        for b in bombs {
            data[b.0][b.1].0 = Cell::Bomb;

            for i in -1..2 {
                for j in -1..2 {
                    if i != 0 || j != 0 {
                        if (b.0 as isize + i) >= 0
                            && (b.0 as isize + i) < size.0 as isize
                            && (b.1 as isize + j) >= 0
                            && (b.1 as isize + j) < size.1 as isize
                        {
                            match data[(b.0 as isize + i) as usize][(b.1 as isize + j) as usize].0 {
                                Cell::Empty(v) => {
                                    data[(b.0 as isize + i) as usize][(b.1 as isize + j) as usize]
                                        .0 = Cell::Empty(v + 1)
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }

        Matrix {
            rows: size.0,
            cols: size.1,
            data: data,
            scale: scale,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn update_state(&mut self, row: usize, col: usize) -> bool {
        if !self.data[row][col].2 {
            self.data[row][col].1 = true;

            if self.data[row][col].0 == Cell::Empty(0) {
                self.reveal_empties((row, col));
            }

            self.data[row][col].0 == Cell::Bomb
        } else {
            false
        }
    }

    pub fn hide(&mut self, row: usize, col: usize) {
        if !self.data[row][col].1 {
            self.data[row][col].2 = !self.data[row][col].2;
        }
    }

    pub fn reveal_empties(&mut self, start: (usize, usize)) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let mut root;
        queue.push_back(start);
        visited.insert(start);

        while !queue.is_empty() {
            match queue.pop_front() {
                Some(pos) => {
                    root = pos;
                    self.data[pos.0][pos.1].1 = true;
                    self.data[pos.0][pos.1].2 = false;
                }

                _ => unreachable!("Queue can not be empty!"),
            }

            if self.data[root.0][root.1].0 == Cell::Empty(0) {
                for i in -1..2 {
                    for j in -1..2 {
                        if i != 0 || j != 0 {
                            if (root.0 as isize + i) >= 0
                                && (root.0 as isize + i) < self.rows as isize
                                && (root.1 as isize + j) >= 0
                                && (root.1 as isize + j) < self.cols as isize
                                && !visited.contains(&(
                                    (root.0 as isize + i) as usize,
                                    (root.1 as isize + j) as usize,
                                ))
                            {
                                queue.push_back((
                                    (root.0 as isize + i) as usize,
                                    (root.1 as isize + j) as usize,
                                ));
                                visited.insert((
                                    (root.0 as isize + i) as usize,
                                    (root.1 as isize + j) as usize,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Drawable for Matrix {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut square =
                    RectangleShape::with_size(Vector2f::new(self.scale - 2.0, self.scale - 2.0));

                square.set_origin(square.size() / 2.0);
                square.set_position((
                    j as f32 * self.scale + self.scale / 2.0,
                    i as f32 * self.scale + self.scale / 2.0,
                ));
                square.set_outline_thickness(1.0);
                square.set_outline_color(Color::rgb(200, 200, 200));
                render_target.draw(&square);
            }
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                match self.data[i][j] {
                    (_, _, true) => {
                        let mut square = RectangleShape::with_size(Vector2f::new(
                            self.scale - 2.0,
                            self.scale - 2.0,
                        ));

                        square.set_origin(square.size() / 2.0);
                        square.set_position((
                            j as f32 * self.scale + self.scale / 2.0,
                            i as f32 * self.scale + self.scale / 2.0,
                        ));
                        square.set_outline_thickness(1.0);
                        square.set_fill_color(Color::rgb(0, 200, 0));
                        square.set_outline_color(Color::rgb(0, 0, 0));
                        render_target.draw(&square);
                    }
                    (Cell::Bomb, true, _) => {
                        let mut square = RectangleShape::with_size(Vector2f::new(
                            self.scale - 2.0,
                            self.scale - 2.0,
                        ));

                        square.set_origin(square.size() / 2.0);
                        square.set_position((
                            j as f32 * self.scale + self.scale / 2.0,
                            i as f32 * self.scale + self.scale / 2.0,
                        ));
                        square.set_outline_thickness(1.0);
                        square.set_fill_color(Color::rgb(255, 0, 0));
                        square.set_outline_color(Color::rgb(0, 0, 0));
                        render_target.draw(&square);
                    }

                    (Cell::Empty(v), true, _) => {
                        let mut square = RectangleShape::with_size(Vector2f::new(
                            self.scale - 2.0,
                            self.scale - 2.0,
                        ));

                        square.set_origin(square.size() / 2.0);
                        square.set_position((
                            j as f32 * self.scale + self.scale / 2.0,
                            i as f32 * self.scale + self.scale / 2.0,
                        ));
                        square.set_outline_thickness(2.0);
                        square.set_fill_color(Color::rgb(125, 125, 125));
                        square.set_outline_color(Color::rgb(200, 200, 200));
                        render_target.draw(&square);

                        if v > 0 {
                            let font = Font::from_file("resources/fonts/mono.ttf").unwrap();
                            let mut text = Text::default();

                            text.set_string(&v.to_string());
                            text.set_font(&font);
                            text.set_fill_color(Color::rgb(0, 0, 0));
                            text.set_character_size(20);
                            let bounds = text.local_bounds();
                            text.set_origin((
                                bounds.left + bounds.width / 2.0,
                                bounds.top + bounds.height / 2.0,
                            ));
                            text.set_position((
                                j as f32 * self.scale + self.scale / 2.0,
                                i as f32 * self.scale + self.scale / 2.0,
                            ));

                            render_target.draw(&text);
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
