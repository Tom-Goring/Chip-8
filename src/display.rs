
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::thread;
use std::sync::mpsc::{TryRecvError};

// use std::io::{self, BufRead};

const PIXEL_SIZE: u32 = 20;
const COLS: usize = 64;
const ROWS: usize = 32;

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
            
             // pixels

            let window_height = 20 * COLS as u32;
            let window_width = 20 * ROWS as u32;

            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("Chip-8", window_height, window_width)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();
            
            let mut event_pump = sdl_context.event_pump().unwrap();

            'main: loop {
                // TODO: Add display drawing here
                canvas.set_draw_color(Color::RGB(255,255,255));
                canvas.clear();

                let mut black = false;
                
                for row in 0..ROWS {
                    black = !black;
                    for col in 0..COLS {
                        let x = col as i32 * PIXEL_SIZE as i32;
                        let y = row as i32 * PIXEL_SIZE as i32;
                        let rect = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);
                        if black {
                            canvas.set_draw_color(Color::RGB(0,0,0));
                            black = !black;
                        } else {
                            canvas.set_draw_color(Color::RGB(255,255,255));
                            black = !black;
                        }
                        canvas.fill_rect(rect);
                    }
                }

                canvas.present();

                // println!("Press enter");
                // let mut line = String::new();
                // let stdin = io::stdin();
                // let _ = stdin.lock().read_line(&mut line);

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