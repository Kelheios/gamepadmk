import { Theme } from "@radix-ui/themes";

import Keyboard from "./Keyboard";
import Settings from "./Settings";

import useSettings, {
  SettingKey,
  SettingType,
  SettingDefault,
} from "./hooks/use-settings";

import "@radix-ui/themes/styles.css";

enum WindowLabel {
  Settings = "settings",
  Keyboard = "keyboard",
}

function App() {
  const urlParams = new URLSearchParams(window.location.search);

  const requestedWindow = urlParams.get("window") ?? WindowLabel.Settings;

  const [darkMode] = useSettings<SettingType[SettingKey.DarkMode]>(
    SettingKey.DarkMode,
    SettingDefault[SettingKey.DarkMode]
  );

  return (
    <Theme appearance={darkMode ? "dark" : "light"}>
      {
        {
          keyboard: <Keyboard />,
          settings: <Settings />,
        }[requestedWindow]
      }
    </Theme>
  );
}

export default App;
