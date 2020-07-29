use sfml::graphics::{Color, RenderTarget, RenderWindow, View};
use sfml::window::{mouse::Button as MouseButton, ContextSettings, Event, Key, Style, VideoMode};

use sfml::system::{Vector2f, Vector2i};

use crate::matrix::Matrix;

pub struct Game {
    map: Matrix,
    window: RenderWindow,
    running: bool,
    scale: f32,
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
        match event {
            // Quit the game is the window is closed
            Event::Closed => self.running = false,

            // Quit the game if escape is pressed
            Event::KeyPressed {
                code: Key::Escape, ..
            } => self.running = false,

            Event::MouseButtonPressed { button, x, y } => {
                let coords = self
                    .window
                    .map_pixel_to_coords_current_view(Vector2i::new(x, y));

                let x = (coords.x as f32 / self.scale) as usize;
                let y = (coords.y as f32 / self.scale) as usize;
                if x < self.map.rows() && y < self.map.cols() {
                    if button == MouseButton::Left {
                        let state = self.map.update_state(y, x);
                    } else if button == MouseButton::Right {
                        self.map.hide(y, x);
                    }
                }
            }

            _ => {}
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let desktop_mode = VideoMode::desktop_mode();
        let mut center = self.window.view().center();
        let view = if width > desktop_mode.width || height > desktop_mode.height {
            center = Vector2f::new(
                center.x.max(desktop_mode.width as f32 / 2.0),
                center.y.max(desktop_mode.height as f32 / 2.0),
            );
            View::new(
                center,
                Vector2f::new(
                    desktop_mode.width as f32 / 2.0,
                    desktop_mode.height as f32 / 2.0,
                ),
            )
        } else {
            center = Vector2f::new(
                if center.x < width as f32 / 2.0 {
                    width as f32 / 2.0
                } else if self.map.cols() as f32 * 64.0 - center.x < width as f32 / 2.0 {
                    self.map.cols() as f32 * 64.0 - width as f32 / 2.0
                } else {
                    center.x
                },
                if center.y < height as f32 / 2.0 {
                    height as f32 / 2.0
                } else if self.map.rows() as f32 * 64.0 - center.y < height as f32 / 2.0 {
                    self.map.rows() as f32 * 64.0 - height as f32 / 2.0
                } else {
                    center.y
                },
            );
            View::new(center, Vector2f::new(width as f32, height as f32))
        };
        self.window.set_view(&view);
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(0, 0, 0));

        self.window.draw(&self.map);

        self.window.display();
    }
}
