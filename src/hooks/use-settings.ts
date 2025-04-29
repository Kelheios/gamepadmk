import { load } from "@tauri-apps/plugin-store";
import { useState, useEffect, useCallback, useMemo } from "react";
import {
  ModeChangeHotKeyButtonsOption,
  SettingKey,
  SettingType,
} from "./types";

const SettingDefault: SettingType = {
  [SettingKey.AutoStart]: false,
  [SettingKey.DarkMode]: false,
  [SettingKey.Keyboard]: true,
  [SettingKey.KeyboardBaseKeySize]: 50,
  [SettingKey.ModeChangeHotKeyButtons]: [ModeChangeHotKeyButtonsOption.Back],
  [SettingKey.ModeChangeHotKeyDurationMs]: 1000,
  [SettingKey.ModeChangeNotification]: true,
  [SettingKey.ModeChangeNotificationDurationMs]: 3000,
};

const SETTINGS_FILE_NAME = "settings.json";

const store = await load(SETTINGS_FILE_NAME);
const savedSettings = await store.entries();

function useSettings<T>(
  key: string,
  defaultValue: T
): [T, (newValue: T) => void] {
  const initialStoredValue = useMemo(() => {
    try {
      const item = savedSettings.find((setting) => setting[0] === key);
      return item ? (item[1] as T) : defaultValue;
    } catch (error) {
      console.error(error);
      return defaultValue;
    }
  }, [key, defaultValue]);

  const [storedValue, setStoredValue] = useState<T>(initialStoredValue);

  useEffect(() => {
    const unlistenPromise = store.onKeyChange(
      key,
      (newValue: T | undefined) => {
        if (newValue !== undefined) {
          setStoredValue(newValue);
        }
      }
    );

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [key]);

  const setValue = useCallback(
    (newValue: T) => {
      setStoredValue(newValue);
      store
        .set(key, newValue)
        .then(() => store.save())
        .catch(console.error);
    },
    [key]
  );

  return [storedValue, setValue];
}

export default useSettings;
export type { SettingType };
export { SettingKey, SettingDefault };
