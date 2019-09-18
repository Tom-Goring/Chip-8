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

    pub fn process_inputs(&mut self) -> Result<[bool; 16], ()> {

        for event in self.events.poll_iter() {
			if let Event::Quit { .. } = event {
                return Err(());
            };
		}

        let pressed_keys: Vec<Keycode> = self.events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let mut keys = [false; 16];

        for key in pressed_keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xD),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xE),
                Keycode::Z => Some(0xA),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xB),
                Keycode::V => Some(0xF),
                _ => None,
            };

            if let Some(i)  = index {
                keys[i] = true;
            }
        }

        Ok(keys)
    }
}