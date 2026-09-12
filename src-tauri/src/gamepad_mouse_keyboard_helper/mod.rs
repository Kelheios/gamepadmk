mod arrow_keys;
mod control_mode;
mod gamepad;
mod key_codes;
mod virtual_keyboard;
mod virtual_mouse;

use crate::helpers::{app_handle_ready, get_app_handle, hide_window, window_exists, AppState};
use crate::setting_manager::{SettingManager, Settings};
use control_mode::{ControlMode, ControlModeState};
use enigo::{Enigo, Keyboard, Mouse};
use gamepad::{GamePad, MotionStateXY, MotionStateZ, PressState};
use key_codes::send_key_action;
use sdl2::event::Event;
use serde::Deserialize;
use serde_json::Value;
use std::{sync::Mutex, time::Duration};
use tauri::{webview::PageLoadEvent, Emitter, Listener, Manager};
use virtual_keyboard::KeyboardNavigationState;
use virtual_mouse::VirtualMouseMoveState;

pub fn release_arrow_keys_on_exit() {
    arrow_keys::shutdown();
}

const REFRESH_RATE: u64 = 1000;
const WAIT_TIME_MS: u64 = 1000 / REFRESH_RATE;

const MOUSE_REFRESH_RATE: u64 = 60;
const MOUSE_WAIT_TIME_MS: u64 = 1000 / MOUSE_REFRESH_RATE;

const KEYBOARD_LONG_HOLD_CLEAR_MS: u64 = 3000;

pub fn init_thread() {
    sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");

    let sdl_context = sdl2::init().unwrap();

    let game_controller_subsystem = sdl_context.game_controller().unwrap();

    let mut controllers = Vec::new();

    let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();

    let mut gamepad_state: GamePad = GamePad::init_state();

    let mut control_mode_state = ControlModeState::init();

    let mut mouse_virtual_mouse_state = VirtualMouseMoveState::init();

    let mut keyboard_virtual_mouse_state = VirtualMouseMoveState::init();

    let mut keyboard_navigation_state = KeyboardNavigationState::init();

    // Register Activey Key Listener
    while !app_handle_ready() {
        std::thread::sleep(Duration::from_millis(100));
    }
    let app = get_app_handle().app_handle();
    app.manage(Mutex::new(AppState::default()));

    // todo may rid of struct, but want to support multi key press so may still need struct
    #[derive(Deserialize)]
    pub struct KeyboardSetActiveKeyEvent {
        pub key: String,
    }
    app.listen("KeyboardSetActiveKey", move |event| {
        let app_state: tauri::State<'_, Mutex<AppState>> =
            app.try_state::<Mutex<AppState>>().unwrap();
        if let Ok(payload) = serde_json::from_str::<KeyboardSetActiveKeyEvent>(&event.payload()) {
            let mut state = app_state.lock().unwrap();
            state.keyboard_mode_active_key = payload.key;
        }
    });

    let setting_manager = SettingManager::init("settings.json");

    let app_state: tauri::State<'_, Mutex<AppState>> = app.try_state::<Mutex<AppState>>().unwrap();
    let mut swapped = false;
    loop {
        if arrow_keys::is_stopped() {
            break;
        }
        if let Some(event) = sdl_context.event_pump().unwrap().poll_event() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    if game_controller_subsystem.is_game_controller(which) {
                        match game_controller_subsystem.open(which) {
                            Ok(controller) => {
                                controllers.push(controller);
                            }
                            Err(e) => println!("failed: {:?}", e),
                        };
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    controllers.retain(|controller| controller.instance_id() != which);
                    arrow_keys::update(&mut enigo, [false; 4]);
                    gamepad_state = GamePad::init_state();
                    keyboard_virtual_mouse_state = VirtualMouseMoveState::init();
                    mouse_virtual_mouse_state.x = 0;
                    mouse_virtual_mouse_state.y = 0;
                }
                Event::ControllerButtonDown { button, .. } => {
                    gamepad_state.buttons.press(button);
                }
                Event::ControllerButtonUp { button, .. } => {
                    gamepad_state.buttons.release(button);
                }
                Event::ControllerAxisMotion { axis, value, .. } => match axis {
                    sdl2::controller::Axis::TriggerLeft | sdl2::controller::Axis::TriggerRight => {
                        gamepad_state.triggers.move_z(axis, value);
                    }
                    sdl2::controller::Axis::LeftX | sdl2::controller::Axis::RightX => {
                        gamepad_state.sticks.move_x(axis, value);
                    }
                    sdl2::controller::Axis::LeftY | sdl2::controller::Axis::RightY => {
                        gamepad_state.sticks.move_y(axis, value);
                    }
                },
                _ => (),
            }
        }

        let hot_key_buttons: Vec<sdl2::controller::Button> = setting_manager
            .get_value(Settings::ModeChangeHotKeyButtons)
            .as_array()
            .unwrap()
            .iter()
            .map(|button| {
                let button_str = button.as_str().unwrap();
                match button_str {
                    "A" => sdl2::controller::Button::A,
                    "B" => sdl2::controller::Button::B,
                    "X" => sdl2::controller::Button::X,
                    "Y" => sdl2::controller::Button::Y,
                    "BACK" => sdl2::controller::Button::Back,
                    "GUIDE" => sdl2::controller::Button::Guide,
                    "START" => sdl2::controller::Button::Start,
                    _ => sdl2::controller::Button::Back,
                }
            })
            .collect();

        if hot_key_buttons.len() > 0
            && gamepad_state.buttons.pressed_for_duration(
                hot_key_buttons.clone(),
                Duration::from_millis(
                    setting_manager
                        .get_value(Settings::ModeChangeHotKeyDurationMs)
                        .as_u64()
                        .unwrap(),
                ),
            )
        {
            control_mode_state.move_to_next_mode();
            gamepad_state.buttons.bulk_ack(hot_key_buttons.clone());
        }

        let next_swapped = setting_manager
            .get_value(Settings::SwapDpadRightStick)
            .as_bool()
            .unwrap_or(false);
        if next_swapped != swapped || control_mode_state.mode == ControlMode::Off {
            arrow_keys::update(&mut enigo, [false; 4]);
            keyboard_virtual_mouse_state.x = 0;
            keyboard_virtual_mouse_state.y = 0;
            if next_swapped != swapped {
                // Re-evaluate held inputs with the new routing, after releasing old output.
                gamepad_state.sticks.right.ack_pending = true;
            }
        }
        if control_mode_state.ack_pending && control_mode_state.mode == ControlMode::GamePadControl
        {
            gamepad_state.sticks.right.ack_pending = true;
        }
        swapped = next_swapped;

        if control_mode_state.mode == ControlMode::GamePadControl {
            match gamepad_state.buttons.a {
                PressState {
                    ack_pending: true,
                    last_pressed: _,
                    pressed,
                    released,
                } => match (pressed, released) {
                    (true, false) => {
                        let _ = enigo.button(enigo::Button::Left, enigo::Direction::Press);
                        gamepad_state.buttons.a.ack();
                    }
                    (false, true) => {
                        let _ = enigo.button(enigo::Button::Left, enigo::Direction::Release);
                        gamepad_state.buttons.a.ack();
                    }
                    _ => (),
                },
                _ => (),
            }

            match gamepad_state.buttons.b {
                PressState {
                    ack_pending: true,
                    last_pressed: _,
                    pressed,
                    released,
                } => match (pressed, released) {
                    (true, false) => {
                        let _ = enigo.button(enigo::Button::Right, enigo::Direction::Press);
                        gamepad_state.buttons.b.ack();
                    }
                    (false, true) => {
                        let _ = enigo.button(enigo::Button::Right, enigo::Direction::Release);
                        gamepad_state.buttons.b.ack();
                    }
                    _ => (),
                },
                _ => (),
            }

            match gamepad_state.buttons.stick_left {
                PressState {
                    ack_pending: true,
                    last_pressed: _,
                    pressed,
                    released,
                } => match (pressed, released) {
                    (false, true) => {
                        mouse_virtual_mouse_state.increase_speed(1, 10);

                        gamepad_state.buttons.stick_left.ack();
                    }
                    _ => (),
                },
                _ => (),
            }

            match gamepad_state.buttons.stick_right {
                PressState {
                    ack_pending: true,
                    last_pressed: _,
                    pressed,
                    released,
                } => match (pressed, released) {
                    (false, true) => {
                        let keyboard_show = setting_manager
                            .get_value(Settings::Keyboard)
                            .as_bool()
                            .unwrap();
                        setting_manager.set("setting_keyboard", Value::Bool(!keyboard_show));

                        gamepad_state.buttons.stick_right.ack();
                    }
                    _ => (),
                },
                _ => (),
            }

            match gamepad_state.sticks.left {
                MotionStateXY {
                    x,
                    y,
                    ack_pending: true,
                } => {
                    mouse_virtual_mouse_state.set_state_from_joystick_input(x, y);

                    gamepad_state.sticks.left.ack();
                }
                _ => (),
            }

            let dpad = [
                gamepad_state.buttons.dpad_up.pressed,
                gamepad_state.buttons.dpad_down.pressed,
                gamepad_state.buttons.dpad_left.pressed,
                gamepad_state.buttons.dpad_right.pressed,
            ];
            if swapped {
                let mut stick = VirtualMouseMoveState::init();
                stick.set_state_from_joystick_input(
                    gamepad_state.sticks.right.x,
                    gamepad_state.sticks.right.y,
                );
                stick.set_state_to_binary_direction();
                arrow_keys::update(&mut enigo, arrow_keys::from_axes(stick.x, stick.y));
                keyboard_virtual_mouse_state.x = i32::from(dpad[3]) - i32::from(dpad[2]);
                keyboard_virtual_mouse_state.y = i32::from(dpad[1]) - i32::from(dpad[0]);
                gamepad_state.sticks.right.ack();
            } else {
                if gamepad_state.sticks.right.ack_pending {
                    keyboard_virtual_mouse_state.set_state_from_joystick_input(
                        gamepad_state.sticks.right.x,
                        gamepad_state.sticks.right.y,
                    );
                    keyboard_virtual_mouse_state.set_state_to_binary_direction();
                    gamepad_state.sticks.right.ack();
                }
                arrow_keys::update(&mut enigo, dpad);
            }
            gamepad_state.buttons.dpad_up.ack();
            gamepad_state.buttons.dpad_down.ack();
            gamepad_state.buttons.dpad_left.ack();
            gamepad_state.buttons.dpad_right.ack();

            match gamepad_state.triggers.left {
                MotionStateZ {
                    z,
                    last_z,
                    ack_pending: true,
                } => {
                    gamepad_state.triggers.left.ack();
                    match (z, last_z) {
                        (z, last_z) if z > 0 && last_z < 1 => {
                            keyboard_navigation_state.shift_on = true;
                            app.emit(
                                "KeyboardShiftIntention",
                                serde_json::to_string(&keyboard_navigation_state.shift_on).unwrap(),
                            )
                            .unwrap();
                        }
                        (z, last_z) if z < 1 && last_z > 0 => {
                            keyboard_navigation_state.shift_on = false;
                            app.emit(
                                "KeyboardShiftIntention",
                                serde_json::to_string(&keyboard_navigation_state.shift_on).unwrap(),
                            )
                            .unwrap();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }

            match gamepad_state.triggers.right {
                MotionStateZ {
                    z,
                    last_z,
                    ack_pending: true,
                } => {
                    gamepad_state.triggers.right.ack();
                    if (z > 0 && last_z < 1) || (z < 1 && last_z > 0) {
                        if setting_manager
                            .get_value(Settings::Keyboard)
                            .as_bool()
                            .unwrap()
                        {
                            let mut state_instance = app_state.lock().unwrap();
                            let direction = if z > 0 {
                                enigo::Direction::Press
                            } else {
                                enigo::Direction::Release
                            };

                            // Emit keyboard press/release intention
                            app.emit(
                                "KeyboardPressIntention",
                                serde_json::to_string(&(direction == enigo::Direction::Press))
                                    .unwrap(),
                            )
                            .unwrap();

                            let active_key = state_instance.keyboard_mode_active_key.clone();

                            // Handle key press/release
                            if direction == enigo::Direction::Press {
                                if !state_instance.keyboard_mode_active_key_held.is_empty()
                                    && state_instance.keyboard_mode_active_key_held != active_key
                                {
                                    send_key_action(
                                        &mut enigo,
                                        state_instance.keyboard_mode_active_key_held.as_str(),
                                        enigo::Direction::Release,
                                    );
                                }
                                send_key_action(
                                    &mut enigo,
                                    active_key.as_str(),
                                    enigo::Direction::Press,
                                );
                                state_instance.keyboard_mode_active_key_held = active_key;
                            } else {
                                send_key_action(
                                    &mut enigo,
                                    state_instance.keyboard_mode_active_key_held.as_str(),
                                    enigo::Direction::Release,
                                );
                                state_instance.keyboard_mode_active_key_held.clear();
                            }
                        }
                    }
                }
                _ => (),
            }

            match gamepad_state.buttons.shoulder_right {
                PressState {
                    ack_pending: true,
                    last_pressed,
                    pressed: _,
                    released,
                } => {
                    gamepad_state.buttons.shoulder_right.ack();

                    if setting_manager
                        .get_value(Settings::Keyboard)
                        .as_bool()
                        .unwrap()
                    {
                        let mut state_instance = app_state.lock().unwrap();
                        let active_key = state_instance.keyboard_mode_active_key.clone();

                        if released {
                            if last_pressed.elapsed().as_millis()
                                < KEYBOARD_LONG_HOLD_CLEAR_MS.into()
                            {
                                // Toggle key state
                                let is_key_active = state_instance
                                    .keyboard_mode_multi_keys_held
                                    .remove(&active_key);
                                send_key_action(
                                    &mut enigo,
                                    active_key.as_str(),
                                    if is_key_active {
                                        enigo::Direction::Release
                                    } else {
                                        enigo::Direction::Press
                                    },
                                );
                                app.emit(
                                    "KeyboardLongPressIntention",
                                    serde_json::to_string(&!is_key_active).unwrap(),
                                )
                                .unwrap();
                                if !is_key_active {
                                    state_instance
                                        .keyboard_mode_multi_keys_held
                                        .insert(active_key.clone());
                                }
                            } else {
                                // Clear all held keys
                                state_instance
                                    .keyboard_mode_multi_keys_held
                                    .iter()
                                    .for_each(|key| {
                                        send_key_action(
                                            &mut enigo,
                                            key.as_str(),
                                            enigo::Direction::Release,
                                        );
                                    });
                                app.emit(
                                    "KeyboardLongPressClearAllIntention",
                                    serde_json::to_string(&true).unwrap(),
                                )
                                .unwrap();
                                state_instance.keyboard_mode_multi_keys_held.clear();
                            }
                        }

                        state_instance.keyboard_mode_multi_keys_held_active = active_key;
                    }
                }
                _ => (),
            }

            if (mouse_virtual_mouse_state.x != 0) || (mouse_virtual_mouse_state.y != 0) {
                if (std::time::Instant::now() - mouse_virtual_mouse_state.last_move).as_millis()
                    > MOUSE_WAIT_TIME_MS as u128
                {
                    mouse_virtual_mouse_state.move_physical_mouse(&mut enigo);
                }
            }
            if (keyboard_virtual_mouse_state.x != 0) || (keyboard_virtual_mouse_state.y != 0) {
                if (std::time::Instant::now() - keyboard_navigation_state.last_navigation)
                    .as_millis()
                    > keyboard_navigation_state.get_speed_ms()
                {
                    keyboard_navigation_state.set_to_next_speed();

                    keyboard_navigation_state.last_navigation = std::time::Instant::now();

                    keyboard_navigation_state.count += 1;
                    keyboard_navigation_state.x = keyboard_virtual_mouse_state.x;
                    keyboard_navigation_state.y = keyboard_virtual_mouse_state.y;
                    app.emit(
                        "KeyboardNavigationIntention",
                        serde_json::to_string(&keyboard_navigation_state).unwrap(),
                    )
                    .unwrap();
                }
            }
        }

        if control_mode_state.ack_pending {
            if setting_manager
                .get_value(Settings::ModeChangeNotification)
                .as_bool()
                .unwrap()
            {
                let show_mode_change_notification_ms = setting_manager
                    .get_value(Settings::ModeChangeNotificationDurationMs)
                    .as_u64()
                    .unwrap();
                std::thread::spawn(move || {
                    while window_exists("toast") {
                        app.get_webview_window("toast").unwrap().destroy().unwrap();
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }

                    let mode_change_toast_window = tauri::WebviewWindowBuilder::new(
                        app,
                        "toast".to_string(),
                        tauri::WebviewUrl::App(
                            format!(
                                "/icons/mode/{}.svg",
                                control_mode_state.get_mode_name().to_ascii_lowercase()
                            )
                            .into(),
                        ),
                    )
                    .inner_size(200.0, 200.0)
                    .visible(false)
                    .center()
                    .decorations(false)
                    .always_on_top(true)
                    .visible_on_all_workspaces(true)
                    .on_page_load(|window, payload| match payload.event() {
                        PageLoadEvent::Finished => {
                            window.show().unwrap();
                        }
                        _ => (),
                    })
                    .build()
                    .unwrap();

                    std::thread::sleep(std::time::Duration::from_millis(
                        show_mode_change_notification_ms,
                    ));

                    mode_change_toast_window.destroy().unwrap();
                });
            }

            match control_mode_state.mode {
                ControlMode::Off => {
                    if window_exists("keyboard") {
                        hide_window("keyboard");
                    }
                }
                ControlMode::GamePadControl => {
                    let keyboard_show = setting_manager
                        .get_value(Settings::Keyboard)
                        .as_bool()
                        .unwrap();
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "keyboard".to_string(),
                        tauri::WebviewUrl::App("index.html?window=keyboard".to_string().into()),
                    )
                    .visible(false)
                    .decorations(false)
                    .always_on_top(true)
                    .visible_on_all_workspaces(true)
                    .on_page_load(move |window, payload| match payload.event() {
                        PageLoadEvent::Finished => {
                            if keyboard_show {
                                window.show().unwrap();
                            }
                        }
                        _ => (),
                    })
                    .inner_size(0.0, 0.0)
                    .build()
                    .unwrap_or_else(|_| {
                        let keyboard_window = app.get_webview_window("keyboard").unwrap();
                        if keyboard_show {
                            keyboard_window.show().unwrap();
                        }
                        keyboard_window
                    });
                }
            }

            control_mode_state.ack();
        }

        std::thread::sleep(std::time::Duration::from_millis(WAIT_TIME_MS));
    }
}
