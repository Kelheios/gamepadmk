import { Flex, Heading, Card } from "@radix-ui/themes";
import { SettingKey } from "../hooks/use-settings";
import {
  enable as systemAutoStartEnable,
  disable as systemAutoStartDisable,
} from "@tauri-apps/plugin-autostart";
import { Toggle } from "./Toggle";
import { NumberSpinner } from "./NumberSpinner";
import { KeyboardKeyPreview } from "./KeyboardKeyPreview";
import { MultiSelect } from "./MultiSelect";
import { ModeChangeHotKeyButtonsOption } from "../hooks/types";

import {
  ButtonA,
  ButtonB,
  ButtonX,
  ButtonY,
  ButtonBack,
  ButtonStart,
} from "../Icons";

const setAutoStart = async (enabled: boolean) =>
  enabled ? await systemAutoStartEnable() : await systemAutoStartDisable();

export default function Settings() {
  return (
    <Flex gap="2" p="2" direction={"column"}>
      <Card>
        <Heading>Settings</Heading>
      </Card>
      <Card>
        <Flex gap="2" direction={"column"}>
          <Toggle
            settingKey={SettingKey.AutoStart}
            label="Automatically start when your computer starts"
            onChange={async (value) => {
              await setAutoStart(value);
            }}
          />

          <Toggle
            settingKey={SettingKey.DarkMode}
            label="Use dark mode for this window and the on screen keyboard"
          />
        </Flex>
      </Card>
      <Card>
        <Flex gap="2" direction={"column"}>
          <Toggle
            settingKey={SettingKey.ModeChangeNotification}
            label="Show a notification when you change modes"
          />

          <NumberSpinner
            settingKey={SettingKey.ModeChangeHotKeyDurationMs}
            label="How many milliseconds hot-keys are held to change modes"
            unit="ms"
            step={100}
          />

          <NumberSpinner
            settingKey={SettingKey.ModeChangeNotificationDurationMs}
            label="How many milliseconds mode change notifications are visibile"
            unit="ms"
            step={100}
          />
          <MultiSelect
            settingKey={SettingKey.ModeChangeHotKeyButtons}
            label="Buttons to hold to change modes"
            options={[
              {
                label: "Back",
                value: ModeChangeHotKeyButtonsOption.Back,
                icon: <ButtonBack size="2rem" />,
              },
              {
                label: "A Button",
                value: ModeChangeHotKeyButtonsOption.A,
                icon: <ButtonA color="#66A14B" size="2rem" />,
              },
              {
                label: "B Button",
                value: ModeChangeHotKeyButtonsOption.B,
                icon: <ButtonB color="#DD322A" size="2rem" />,
              },
              {
                label: "Start",
                value: ModeChangeHotKeyButtonsOption.Start,
                icon: <ButtonStart size="2rem" />,
              },
              {
                label: "X Button",
                value: ModeChangeHotKeyButtonsOption.X,
                icon: <ButtonX color="#1D47C5" size="2rem" />,
              },
              {
                label: "Y Button",
                value: ModeChangeHotKeyButtonsOption.Y,
                icon: <ButtonY color="#FFC102" size="2rem" />,
              },
            ]}
          />
        </Flex>
      </Card>
      <Card>
        <Flex gap="2" direction={"column"}>
          <Toggle
            settingKey={SettingKey.SwapDpadRightStick}
            label="Swap D-pad and Right Stick"
          />
          <Toggle
            settingKey={SettingKey.Keyboard}
            label="Show on screen keyboard when control mode is active"
          />
          <NumberSpinner
            settingKey={SettingKey.KeyboardBaseKeySize}
            label="Size of on screen keyboard keys"
            unit="px"
            step={1}
          />

          <Card>
            <KeyboardKeyPreview keys={["q", `w`, `e`, `r`, `t`, `y`]} />
          </Card>
        </Flex>
      </Card>
    </Flex>
  );
}
