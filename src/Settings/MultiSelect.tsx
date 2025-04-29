import { CheckboxGroup, Flex, Grid, Text } from "@radix-ui/themes";
import useSettings, { SettingDefault, SettingKey } from "../hooks/use-settings";

export const MultiSelect = ({
  settingKey,
  label,
  options,
}: {
  settingKey: SettingKey;
  label: string;
  options: {
    label: string;
    value: string;
    icon?: JSX.Element | null | undefined;
  }[];
}) => {
  const [setting, updateSetting] = useSettings<string[]>(
    settingKey,
    SettingDefault[settingKey] as string[]
  );

  return (
    <>
      <Text as="label" size="4">
        {label}
      </Text>
      <CheckboxGroup.Root defaultValue={setting} size="3">
        <Grid gap="2" columns="3">
          {options.map((option) => (
            <Flex key={option.value}>
              <CheckboxGroup.Item
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
                key={option.value}
                value={option.value}
                onClick={() =>
                  updateSetting(
                    setting.includes(option.value)
                      ? setting.filter((item) => item !== option.value)
                      : [...setting, option.value]
                  )
                }
              >
                <Flex gap="2" align="center">
                  {option.icon}
                  {option.label}
                </Flex>
              </CheckboxGroup.Item>
            </Flex>
          ))}
        </Grid>
      </CheckboxGroup.Root>
    </>
  );
};
