import { Flex, Switch, Text } from "@radix-ui/themes";
import useSettings, { SettingKey, SettingDefault } from "../hooks/use-settings";

export const Toggle = ({
  settingKey,
  label,
  onChange,
}: {
  settingKey: SettingKey;
  label: string;
  onChange?: (value: boolean) => void;
}) => {
  const [setting, updateSetting] = useSettings<boolean>(
    settingKey,
    SettingDefault[settingKey] as boolean
  );

  return (
    <Text as="label" size="4">
      <Flex gap="2">
        <Switch
          size="3"
          checked={setting}
          onClick={async () => {
            const value = !setting;
            updateSetting(value);
            onChange && onChange(value);
          }}
        />
        {label}
      </Flex>
    </Text>
  );
};
