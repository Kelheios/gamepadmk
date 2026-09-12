use enigo::{Direction, Enigo, Key, Keyboard};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Shared with the Tauri exit callback so shutdown cannot race a new key press.
static OUTPUT: Lazy<Mutex<ArrowState>> = Lazy::new(|| Mutex::new(ArrowState::default()));
const KEYS: [Key; 4] = [
    Key::UpArrow,
    Key::DownArrow,
    Key::LeftArrow,
    Key::RightArrow,
];

#[derive(Default)]
struct ArrowState {
    held: [bool; 4],
    stopped: bool,
}

pub fn from_axes(x: i32, y: i32) -> [bool; 4] {
    [y < 0, y > 0, x < 0, x > 0]
}

impl ArrowState {
    fn update(&mut self, desired: [bool; 4], mut send: impl FnMut(usize, bool) -> bool) {
        let desired = if self.stopped { [false; 4] } else { desired };
        // Release ALL old directions before pressing any new ones, including reversals.
        for press in [false, true] {
            for (index, wanted) in desired.iter().copied().enumerate() {
                if press && self.held[index ^ 1] {
                    continue;
                }
                if wanted == press && self.held[index] != wanted && send(index, wanted) {
                    self.held[index] = wanted;
                }
            }
        }
    }
}

pub fn update(enigo: &mut Enigo, desired: [bool; 4]) {
    OUTPUT.lock().unwrap().update(desired, |index, press| {
        enigo
            .key(
                KEYS[index],
                if press {
                    Direction::Press
                } else {
                    Direction::Release
                },
            )
            .is_ok()
    });
}

pub fn is_stopped() -> bool {
    OUTPUT.lock().unwrap().stopped
}

pub fn shutdown() {
    let mut state = OUTPUT.lock().unwrap();
    state.stopped = true;
    if let Ok(mut enigo) = Enigo::new(&enigo::Settings::default()) {
        state.update([false; 4], |index, _| {
            enigo.key(KEYS[index], Direction::Release).is_ok()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_support_neutral_cardinals_and_diagonals() {
        assert_eq!(from_axes(0, 0), [false; 4]);
        assert_eq!(from_axes(1, -1), [true, false, false, true]);
        assert_eq!(from_axes(-1, 1), [false, true, true, false]);
    }

    #[test]
    fn hold_reverse_and_release_emit_only_transitions() {
        let mut state = ArrowState::default();
        let mut events = Vec::new();
        for axes in [(1, 0), (1, 0), (-1, 0), (0, 0)] {
            state.update(from_axes(axes.0, axes.1), |key, press| {
                events.push((key, press));
                true
            });
        }
        assert_eq!(events, vec![(3, true), (3, false), (2, true), (2, false)]);
        assert_eq!(state.held, [false; 4]);
    }

    #[test]
    fn shutdown_releases_diagonals_and_prevents_repress() {
        let mut state = ArrowState::default();
        state.update(from_axes(1, -1), |_, _| true);
        state.stopped = true;
        let mut events = Vec::new();
        state.update(from_axes(1, -1), |key, press| {
            events.push((key, press));
            true
        });
        assert_eq!(events, vec![(0, false), (3, false)]);
        assert_eq!(state.held, [false; 4]);
    }

    #[test]
    fn failed_release_is_retried() {
        let mut state = ArrowState::default();
        state.update(from_axes(1, 0), |_, _| true);
        state.update([false; 4], |_, _| false);
        assert!(state.held[3]);
        state.update([false; 4], |_, _| true);
        assert_eq!(state.held, [false; 4]);
    }
}
