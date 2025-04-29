use serde::Serialize;

#[derive(Serialize)]
pub struct PressState {
    #[serde(skip_serializing)]
    pub ack_pending: bool,
    #[serde(skip_serializing)]
    pub last_pressed: std::time::Instant,
    #[serde(skip_serializing)]
    pub pressed: bool,
    #[serde(skip_serializing)]
    pub released: bool,
}

impl PressState {
    pub fn ack(&mut self) {
        self.ack_pending = false;
    }
}

#[derive(Serialize)]
pub struct MotionStateZ {
    pub z: i16,
    pub last_z: i16,
    #[serde(skip_serializing)]
    pub ack_pending: bool,
}

impl MotionStateZ {
    pub fn ack(&mut self) {
        self.ack_pending = false;
    }
}

#[derive(Serialize)]
pub struct MotionStateXY {
    pub x: i16,
    pub y: i16,
    #[serde(skip_serializing)]
    pub ack_pending: bool,
}

impl MotionStateXY {
    pub fn ack(&mut self) {
        self.ack_pending = false;
    }
}

#[derive(Serialize)]
pub struct Buttons {
    pub a: PressState,
    pub b: PressState,
    pub x: PressState,
    pub y: PressState,
    pub back: PressState,
    pub guide: PressState,
    pub start: PressState,
    pub stick_left: PressState,
    pub stick_right: PressState,
    pub shoulder_left: PressState,
    pub shoulder_right: PressState,
    pub dpad_up: PressState,
    pub dpad_down: PressState,
    pub dpad_left: PressState,
    pub dpad_right: PressState,
}

impl Buttons {
    fn init_button_state() -> PressState {
        PressState {
            ack_pending: false,
            last_pressed: std::time::Instant::now(),
            pressed: false,
            released: false,
        }
    }
    fn init_state() -> Buttons {
        Buttons {
            a: Self::init_button_state(),
            b: Self::init_button_state(),
            x: Self::init_button_state(),
            y: Self::init_button_state(),
            back: Self::init_button_state(),
            guide: Self::init_button_state(),
            start: Self::init_button_state(),
            stick_left: Self::init_button_state(),
            stick_right: Self::init_button_state(),
            shoulder_left: Self::init_button_state(),
            shoulder_right: Self::init_button_state(),
            dpad_up: Self::init_button_state(),
            dpad_down: Self::init_button_state(),
            dpad_left: Self::init_button_state(),
            dpad_right: Self::init_button_state(),
        }
    }
    fn get_button(&self, button: sdl2::controller::Button) -> Option<&PressState> {
        match button {
            sdl2::controller::Button::A => Some(&self.a),
            sdl2::controller::Button::B => Some(&self.b),
            sdl2::controller::Button::X => Some(&self.x),
            sdl2::controller::Button::Y => Some(&self.y),
            sdl2::controller::Button::Back => Some(&self.back),
            sdl2::controller::Button::Guide => Some(&self.guide),
            sdl2::controller::Button::Start => Some(&self.start),
            sdl2::controller::Button::LeftStick => Some(&self.stick_left),
            sdl2::controller::Button::RightStick => Some(&self.stick_right),
            sdl2::controller::Button::LeftShoulder => Some(&self.shoulder_left),
            sdl2::controller::Button::RightShoulder => Some(&self.shoulder_right),
            sdl2::controller::Button::DPadUp => Some(&self.dpad_up),
            sdl2::controller::Button::DPadDown => Some(&self.dpad_down),
            sdl2::controller::Button::DPadLeft => Some(&self.dpad_left),
            sdl2::controller::Button::DPadRight => Some(&self.dpad_right),
            _ => None,
        }
    }
    fn set_button(&mut self, button: sdl2::controller::Button, state: PressState) {
        match button {
            sdl2::controller::Button::A => self.a = state,
            sdl2::controller::Button::B => self.b = state,
            sdl2::controller::Button::X => self.x = state,
            sdl2::controller::Button::Y => self.y = state,
            sdl2::controller::Button::Back => self.back = state,
            sdl2::controller::Button::Guide => self.guide = state,
            sdl2::controller::Button::Start => self.start = state,
            sdl2::controller::Button::LeftStick => self.stick_left = state,
            sdl2::controller::Button::RightStick => self.stick_right = state,
            sdl2::controller::Button::LeftShoulder => self.shoulder_left = state,
            sdl2::controller::Button::RightShoulder => self.shoulder_right = state,
            sdl2::controller::Button::DPadUp => self.dpad_up = state,
            sdl2::controller::Button::DPadDown => self.dpad_down = state,
            sdl2::controller::Button::DPadLeft => self.dpad_left = state,
            sdl2::controller::Button::DPadRight => self.dpad_right = state,
            _ => (),
        }
    }
    pub fn press(&mut self, button: sdl2::controller::Button) {
        self.set_button(
            button,
            PressState {
                ack_pending: true,
                last_pressed: std::time::Instant::now(),
                pressed: true,
                released: false,
            },
        );
    }
    pub fn release(&mut self, button: sdl2::controller::Button) {
        self.set_button(
            button,
            PressState {
                ack_pending: true,
                last_pressed: self.get_button(button).unwrap().last_pressed,
                pressed: false,
                released: true,
            },
        );
    }
    pub fn pressed_for_duration(
        &self,
        buttons: Vec<sdl2::controller::Button>,
        duration: std::time::Duration,
    ) -> bool {
        for button in buttons {
            match button {
                sdl2::controller::Button::A => {
                    if !self.a.ack_pending
                        || !self.b.pressed
                        || self.a.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::B => {
                    if !self.b.ack_pending
                        || !self.b.pressed
                        || self.b.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::X => {
                    if !self.x.ack_pending
                        || !self.x.pressed
                        || self.x.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::Y => {
                    if !self.y.ack_pending
                        || !self.y.pressed
                        || self.y.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::Back => {
                    if !self.back.ack_pending
                        || !self.back.pressed
                        || self.back.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::Guide => {
                    if !self.guide.ack_pending
                        || !self.guide.pressed
                        || self.guide.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                sdl2::controller::Button::Start => {
                    if !self.start.ack_pending
                        || !self.start.pressed
                        || self.start.last_pressed.elapsed() < duration
                    {
                        return false;
                    }
                }
                _ => (),
            }
        }
        true
    }

    pub fn bulk_ack(&mut self, buttons: Vec<sdl2::controller::Button>) {
        for button in buttons {
            match button {
                sdl2::controller::Button::A => self.a.ack_pending = false,
                sdl2::controller::Button::B => self.b.ack_pending = false,
                sdl2::controller::Button::X => self.x.ack_pending = false,
                sdl2::controller::Button::Y => self.y.ack_pending = false,
                sdl2::controller::Button::Back => self.back.ack_pending = false,
                sdl2::controller::Button::Guide => self.guide.ack_pending = false,
                sdl2::controller::Button::Start => self.start.ack_pending = false,
                _ => (),
            }
        }
    }
}

#[derive(Serialize)]
pub struct Triggers {
    pub left: MotionStateZ,
    pub right: MotionStateZ,
}

impl Triggers {
    fn init_trigger_state() -> MotionStateZ {
        MotionStateZ {
            z: 0,
            last_z: 0,
            ack_pending: false,
        }
    }
    fn init_state() -> Triggers {
        Triggers {
            left: Self::init_trigger_state(),
            right: Self::init_trigger_state(),
        }
    }
    fn set_trigger(&mut self, trigger: sdl2::controller::Axis, state: MotionStateZ) {
        match trigger {
            sdl2::controller::Axis::TriggerLeft => self.left = state,
            sdl2::controller::Axis::TriggerRight => self.right = state,
            _ => (),
        }
    }
    pub fn move_z(&mut self, trigger: sdl2::controller::Axis, z: i16) {
        self.set_trigger(
            trigger,
            MotionStateZ {
                z,
                last_z: match trigger {
                    sdl2::controller::Axis::TriggerLeft => self.left.z,
                    sdl2::controller::Axis::TriggerRight => self.right.z,
                    _ => 0,
                },
                ack_pending: true,
            },
        );
    }
}

#[derive(Serialize)]
pub struct Sticks {
    pub left: MotionStateXY,
    pub right: MotionStateXY,
}

impl Sticks {
    fn init_stick_state() -> MotionStateXY {
        MotionStateXY {
            x: 0,
            y: 0,
            ack_pending: false,
        }
    }
    fn init_state() -> Sticks {
        Sticks {
            left: Self::init_stick_state(),
            right: Self::init_stick_state(),
        }
    }
    fn set_stick(&mut self, stick: sdl2::controller::Axis, state: MotionStateXY) {
        match stick {
            sdl2::controller::Axis::LeftX => self.left = state,
            sdl2::controller::Axis::LeftY => self.left = state,
            sdl2::controller::Axis::RightX => self.right = state,
            sdl2::controller::Axis::RightY => self.right = state,
            _ => (),
        }
    }
    pub fn move_x(&mut self, stick: sdl2::controller::Axis, x: i16) {
        self.set_stick(
            stick,
            MotionStateXY {
                x,
                y: match stick {
                    sdl2::controller::Axis::LeftX => self.left.y,
                    sdl2::controller::Axis::RightX => self.right.y,
                    _ => 0,
                },
                ack_pending: true,
            },
        );
    }
    pub fn move_y(&mut self, stick: sdl2::controller::Axis, y: i16) {
        self.set_stick(
            stick,
            MotionStateXY {
                x: match stick {
                    sdl2::controller::Axis::LeftY => self.left.x,
                    sdl2::controller::Axis::RightY => self.right.x,
                    _ => 0,
                },
                y,
                ack_pending: true,
            },
        );
    }
}

#[derive(Serialize)]
pub struct GamePad {
    pub buttons: Buttons,
    pub triggers: Triggers,
    pub sticks: Sticks,
}

impl GamePad {
    pub fn init_state() -> GamePad {
        GamePad {
            buttons: Buttons::init_state(),
            triggers: Triggers::init_state(),
            sticks: Sticks::init_state(),
        }
    }
}
