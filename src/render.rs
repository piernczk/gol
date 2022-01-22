use crate::board;
use crate::misc;
use sdl2::{
    video::Window,
    render::Canvas,
    rect::Rect,
    pixels::Color,
    keyboard::Keycode,
    event::Event,
    EventPump
};

/// Used to render the window.
pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub cell_size: u16,
    pub dead_cell: (u8, u8, u8),
    pub live_cell: (u8, u8, u8),
}

impl Renderer {
    /// Constructs a new renderer.
    pub fn new(cell_size: u16, board_size: misc::Point) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("game of life", 
            board_size.x as u32 * cell_size as u32,
            board_size.y as u32 * cell_size as u32
        ).position_centered().build().unwrap();

        Self { 
            canvas: window.into_canvas().present_vsync().build().unwrap(),
            event_pump,
            cell_size,
            dead_cell: (0, 0, 0),
            live_cell: (255, 255, 255)
        }
    }

    /// Renders the window.
    pub fn render(&mut self, board: &board::Board) {
        self.canvas.set_draw_color(Color::from(self.dead_cell));
        self.canvas.clear();
        
        self.canvas.set_draw_color(Color::from(self.live_cell));
        for x in 0..board.get_size().x {
            for y in 0..board.get_size().y {
                if board.get((x, y).into()) {
                    let rect = Rect::new(
                        x as i32 * self.cell_size as i32,
                        y as i32 * self.cell_size as i32,
                        self.cell_size as u32, self.cell_size as u32
                    );
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }

        self.canvas.present();
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                _ => {}
            }
        }
    }
}
