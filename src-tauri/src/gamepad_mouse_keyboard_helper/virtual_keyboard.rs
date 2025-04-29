use serde::Serialize;
pub enum KeyboardNavigationSpeed {
    Slow,
    Medium,
    Fast,
}

#[derive(Serialize)]
pub struct KeyboardNavigationState {
    pub x: i32,
    pub y: i32,
    pub count: i64,
    pub shift_on: bool,
    #[serde(skip_serializing)]
    pub last_navigation: std::time::Instant,
    #[serde(skip_serializing)]
    pub speed: KeyboardNavigationSpeed,
    #[serde(skip_serializing)]
    speed_reset_ms: u128,
}

impl KeyboardNavigationState {
    pub fn init() -> Self {
        Self {
            x: 0,
            y: 0,
            count: 0,
            shift_on: false,
            last_navigation: std::time::Instant::now(),
            speed: KeyboardNavigationSpeed::Slow,
            speed_reset_ms: 5000,
        }
    }
    pub fn get_speed_ms(&self) -> u128 {
        match self.speed {
            KeyboardNavigationSpeed::Slow => 400,
            KeyboardNavigationSpeed::Medium => 300,
            KeyboardNavigationSpeed::Fast => 200,
        }
    }
    fn get_next_speed(&self) -> KeyboardNavigationSpeed {
        match self.speed {
            KeyboardNavigationSpeed::Slow => KeyboardNavigationSpeed::Medium,
            KeyboardNavigationSpeed::Medium => KeyboardNavigationSpeed::Fast,
            KeyboardNavigationSpeed::Fast => {
                if (std::time::Instant::now() - self.last_navigation).as_millis()
                    > self.speed_reset_ms
                {
                    KeyboardNavigationSpeed::Slow
                } else {
                    KeyboardNavigationSpeed::Fast
                }
            }
        }
    }
    pub fn set_to_next_speed(&mut self) {
        self.speed = self.get_next_speed();
    }
}
