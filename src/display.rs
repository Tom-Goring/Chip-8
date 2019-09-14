
//use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::thread;
use std::sync::mpsc::{TryRecvError};

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 600;
const ROWS: usize = 64;
const COLS: usize = 32;

pub struct Display {
    pub buffer: [bool; ROWS * COLS],
}

impl Display {
    pub fn new() -> Display {
        let array: [bool; ROWS * COLS] = [false; ROWS * COLS];
        Display {
            buffer: array,
        }
    }

    pub fn rows() -> usize {
        ROWS
    }

    pub fn cols() -> usize {
        COLS
    }

    pub fn display(&mut self, tx: std::sync::mpsc::Sender<bool>, rx: std::sync::mpsc::Receiver<bool>) -> std::thread::JoinHandle<()> {

        thread::spawn(move || {
            
            let rect_height = WINDOW_HEIGHT / ROWS as u32;
            let rect_width = WINDOW_WIDTH / COLS as u32;

            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("Chip-8", WINDOW_HEIGHT, WINDOW_WIDTH)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();
            
            let mut event_pump = sdl_context.event_pump().unwrap();

            'main: loop {
                // TODO: Add display drawing here
                canvas.set_draw_color(Color::RGB(255,255,255));
                canvas.clear();
                canvas.present();

                for event in event_pump.poll_iter() {
                    if let Event::Quit {..} = event { 
                        tx.send(false);
                        break 'main 
                    }
                }
                
                match rx.try_recv() {
                    Ok(true) | Err(TryRecvError::Disconnected) => {
                        break 'main;
                    }
                    Err(TryRecvError::Empty) | Ok(false) => {}
                }
            }
        })
    }
}