export enum SettingKey {
  AutoStart = "setting_auto_start",
  DarkMode = "setting_dark_mode",
  SwapDpadRightStick = "setting_swap_dpad_right_stick",
  Keyboard = "setting_keyboard",
  KeyboardBaseKeySize = "setting_keyboard_base_key_size",
  ModeChangeHotKeyButtons = "setting_mode_change_hot_key_buttons",
  ModeChangeHotKeyDurationMs = "setting_mode_change_hot_key_duration_ms",
  ModeChangeNotification = "setting_mode_change_notification",
  ModeChangeNotificationDurationMs = "setting_mode_change_notification_duration_ms",
}

export interface SettingType {
  [SettingKey.AutoStart]: boolean;
  [SettingKey.DarkMode]: boolean;
  [SettingKey.SwapDpadRightStick]: boolean;
  [SettingKey.Keyboard]: boolean;
  [SettingKey.KeyboardBaseKeySize]: number;
  [SettingKey.ModeChangeHotKeyButtons]: string[];
  [SettingKey.ModeChangeHotKeyDurationMs]: number;
  [SettingKey.ModeChangeNotification]: boolean;
  [SettingKey.ModeChangeNotificationDurationMs]: number;
}

export enum ModeChangeHotKeyButtonsOption {
  A = "A",
  B = "B",
  X = "X",
  Y = "Y",
  Back = "BACK",
  Guide = "GUIDE",
  Start = "START",
}
