import { CircleMinus, CirclePlus } from "../Icons";
import { TextField, IconButton, Flex, Text } from "@radix-ui/themes";
import useSettings, { SettingKey, SettingDefault } from "../hooks/use-settings";

export const NumberSpinner = ({
  settingKey,
  label,
  unit,
  step = 1,
}: {
  settingKey: SettingKey;
  label: string;
  unit?: string;
  step?: number;
}) => {
  const [setting, updateSetting] = useSettings<number>(
    settingKey,
    SettingDefault[settingKey] as number
  );

  return (
    <Text as="label" size="4">
      <Flex gap="2" align={"center"}>
        <TextField.Root
          size="2"
          style={{ width: "9rem" }}
          value={setting}
          radius="full"
          onChange={(e) => {
            const value = parseInt(e.target.value);
            !isNaN(value) && updateSetting(value);
          }}
        >
          <TextField.Slot>
            <IconButton
              variant="ghost"
              onClick={() => {
                updateSetting(Math.max(setting - step, 0));
              }}
            >
              <CircleMinus size={"1.25rem"} thickness={8} />
            </IconButton>
          </TextField.Slot>
          <TextField.Slot>
            {unit}
            <IconButton
              variant="ghost"
              onClick={() => {
                updateSetting(setting + step);
              }}
            >
              <CirclePlus size={"1.25rem"} thickness={8} />
            </IconButton>
          </TextField.Slot>
        </TextField.Root>
        {label}
      </Flex>
    </Text>
  );
};
