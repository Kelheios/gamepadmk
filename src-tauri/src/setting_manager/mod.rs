use std::sync::Arc;

use tauri::{Manager, Wry};

use serde_json::Value;
use tauri_plugin_store::StoreExt;

use crate::helpers::get_app_handle;

pub enum Settings {
    Keyboard,
    ModeChangeHotKeyButtons,
    ModeChangeHotKeyDurationMs,
    ModeChangeNotification,
    ModeChangeNotificationDurationMs,
}

pub struct SettingManager {
    store: Result<Arc<tauri_plugin_store::Store<Wry>>, tauri_plugin_store::Error>,
}

impl SettingManager {
    pub fn init(filename: &str) -> SettingManager {
        let store = get_app_handle().app_handle().store(filename);

        SettingManager { store }
    }

    fn defaults(&self) -> Value {
        let mut defaults = Value::default();
        defaults["setting_keyboard"] = Value::Bool(true);
        defaults["setting_mode_change_hot_key_buttons"] =
            Value::Array(vec![Value::String("BACK".to_string())]);
        defaults["setting_mode_change_hot_key_duration_ms"] = Value::from(1000);
        defaults["setting_mode_change_notification"] = Value::Bool(true);
        defaults["setting_mode_change_notification_duration_ms"] = Value::from(3000);
        defaults
    }

    pub fn get_value(&self, key: Settings) -> Value {
        match key {
            Settings::Keyboard => self.get("setting_keyboard"),
            Settings::ModeChangeHotKeyButtons => self.get("setting_mode_change_hot_key_buttons"),
            Settings::ModeChangeHotKeyDurationMs => {
                self.get("setting_mode_change_hot_key_duration_ms")
            }
            Settings::ModeChangeNotification => self.get("setting_mode_change_notification"),
            Settings::ModeChangeNotificationDurationMs => {
                self.get("setting_mode_change_notification_duration_ms")
            }
        }
    }

    fn get(&self, key: &str) -> Value {
        let store = self.store.as_ref().unwrap();
        match store.get(key) {
            Option::Some(value) => Value::from(value),
            Option::None => self.defaults()[key].clone(),
        }
    }

    pub fn set(&self, key: &str, value: Value) {
        let store = self.store.as_ref().unwrap();
        store.set(key, value);
    }
}
