use enigo::{Enigo, Mouse};

pub struct VirtualMouseMoveState {
    pub x: i32,
    pub y: i32,
    pub last_move: std::time::Instant,
    speed: i16,
}

impl VirtualMouseMoveState {
    pub fn init() -> VirtualMouseMoveState {
        VirtualMouseMoveState {
            x: 0,
            y: 0,
            last_move: std::time::Instant::now(),
            speed: 1,
        }
    }
    pub fn increase_speed(&mut self, step: i16, max_speed: i16) {
        let new_speed = if self.speed >= max_speed {
            1
        } else {
            self.speed + step
        };
        self.speed = new_speed;
    }
    pub fn move_physical_mouse(&mut self, enigo: &mut Enigo) {
        self.last_move = std::time::Instant::now();
        enigo
            .move_mouse(self.x, self.y, enigo::Coordinate::Rel)
            .unwrap();
    }
    fn translate_joystick_input_to_mouse_state(input: i16, deadzone: u16, speed: u16) -> i32 {
        let abs_input = input.unsigned_abs() as u16;

        if abs_input <= deadzone {
            return 0;
        }

        let delta_abs = abs_input - deadzone;

        let scaled_value = ((delta_abs as i32) * (speed as i32)) / (10000 as i32);

        if input < 0 {
            return -scaled_value;
        } else {
            return scaled_value;
        }
    }
    pub fn set_state_from_joystick_input(&mut self, x: i16, y: i16) {
        self.x = VirtualMouseMoveState::translate_joystick_input_to_mouse_state(
            x,
            2000,
            self.speed as u16,
        );
        self.y = VirtualMouseMoveState::translate_joystick_input_to_mouse_state(
            y,
            2000,
            self.speed as u16,
        );
    }
    pub fn set_state_to_binary_direction(&mut self) {
        if self.x != 0 {
            self.x = self.x / (self.x.abs());
        }
        if self.y != 0 {
            self.y = self.y / (self.y.abs());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_direction_preserves_existing_deadzone_and_scaling() {
        let mut state = VirtualMouseMoveState::init();
        for input in [0, 2000, -2000, 11999, -11999] {
            state.set_state_from_joystick_input(input, input);
            state.set_state_to_binary_direction();
            assert_eq!((state.x, state.y), (0, 0));
        }
        state.set_state_from_joystick_input(12000, -12000);
        state.set_state_to_binary_direction();
        assert_eq!((state.x, state.y), (1, -1));
        state.set_state_from_joystick_input(i16::MIN, i16::MAX);
        state.set_state_to_binary_direction();
        assert_eq!((state.x, state.y), (-1, 1));
    }
}
