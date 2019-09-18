use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver {
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn process_inputs(&mut self) {

        for event in events.poll_iter() {
				match event {
					Event::Quit {..} => break 'main,
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                	},
					_ => {}
				}
			}
    }
}