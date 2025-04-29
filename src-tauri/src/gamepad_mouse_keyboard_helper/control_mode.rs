#[derive(Copy, Clone, PartialEq)]
pub enum ControlMode {
    Off,
    GamePadControl,
}

#[derive(Copy, Clone)]
pub struct ControlModeState {
    pub mode: ControlMode,
    pub ack_pending: bool,
}

impl ControlModeState {
    pub fn init() -> ControlModeState {
        ControlModeState {
            mode: ControlMode::Off,
            ack_pending: false,
        }
    }
    fn set_mode(&mut self, mode: ControlMode) {
        self.mode = mode;
        self.ack_pending = true;
    }
    pub fn get_mode_name(&self) -> String {
        match self.mode {
            ControlMode::Off => "Off".to_string(),
            ControlMode::GamePadControl => "Control".to_string(),
        }
    }
    pub fn move_to_next_mode(&mut self) {
        match self.mode {
            ControlMode::Off => self.set_mode(ControlMode::GamePadControl),
            ControlMode::GamePadControl => self.set_mode(ControlMode::Off),
        }
    }
    pub fn ack(&mut self) {
        self.ack_pending = false;
    }
}
