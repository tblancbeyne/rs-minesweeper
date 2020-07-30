use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable};
use sfml::window::{mouse::Button as MouseButton, ContextSettings, Event, Key, Style};

use sfml::system::Vector2i;

use crate::matrix::Matrix;

pub struct Game {
    map: Matrix,
    window: RenderWindow,
    running: bool,
    scale: f32,
    finished: (bool, bool),
}

impl Game {
    pub fn new(rows: usize, cols: usize, scale: usize, bombs: usize) -> Self {
        let window = RenderWindow::new(
            ((cols * scale) as u32, (scale * cols) as u32),
            "Simple minesweeper",
            Style::TITLEBAR,
            &ContextSettings::default(),
        );

        Game {
            map: Matrix::new((rows, cols, bombs), scale as f32),
            window: window,
            running: true,
            scale: scale as f32,
            finished: (false, false),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            self.manage_event(event);
        }
    }

    fn manage_event(&mut self, event: Event) {
        match (event, self.finished) {
            // Quit the game is the window is closed
            (Event::Closed, _) => self.running = false,

            // Quit the game if escape is pressed
            (
                Event::KeyPressed {
                    code: Key::Escape, ..
                },
                _,
            ) => self.running = false,

            (
                Event::KeyPressed {
                    code: Key::Return, ..
                },
                _,
            ) => {
                self.map = Matrix::new(
                    (self.map.rows(), self.map.cols(), self.map.bombs()),
                    self.scale,
                );
                self.finished = (false, false);
            }

            (Event::MouseButtonPressed { button, x, y }, (false, _)) => {
                let coords = self
                    .window
                    .map_pixel_to_coords_current_view(Vector2i::new(x, y));

                let x = (coords.x as f32 / self.scale) as usize;
                let y = (coords.y as f32 / self.scale) as usize;
                if x < self.map.rows() && y < self.map.cols() {
                    if button == MouseButton::Left {
                        if self.map.update_state(y, x) {
                            self.finished = (true, false);
                        } else if self.map.is_finished() {
                            self.finished = (true, true);
                        }
                    } else if button == MouseButton::Right {
                        self.map.hide(y, x);
                    }
                }
            }

            _ => {}
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(0, 0, 0));

        self.window.draw(&self.map);

        if self.finished.0 {
            self.draw_message(if self.finished.1 {
                "You won!".to_owned()
            } else {
                "You lost!".to_owned()
            })
        }

        self.window.display();
    }

    fn draw_message(&mut self, v: String) {
        let font = Font::from_file("resources/fonts/mono.ttf").unwrap();
        let mut text = Text::default();

        text.set_string(&v);
        text.set_font(&font);
        text.set_fill_color(Color::rgb(0, 0, 0));
        text.set_outline_thickness(2.0);
        text.set_outline_color(Color::rgb(255, 255, 255));
        text.set_character_size(30);
        let bounds = text.local_bounds();
        text.set_origin((
            bounds.left + bounds.width / 2.0,
            bounds.top + bounds.height / 2.0,
        ));
        text.set_position((
            self.map.cols() as f32 * self.scale / 2.0,
            self.map.rows() as f32 * self.scale / 2.0,
        ));

        self.window.draw(&text);
    }
}
