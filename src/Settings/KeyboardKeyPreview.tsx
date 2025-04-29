import { Flex } from "@radix-ui/themes";
import { KeyFaceContainer, KeyFace } from "../Keyboard";
import useSettings, { SettingKey, SettingDefault } from "../hooks/use-settings";

export const KeyboardKeyPreview = ({ keys }: { keys: string[] }) => {
  const [getBaseKeySize] = useSettings<number>(
    SettingKey.KeyboardBaseKeySize,
    SettingDefault[SettingKey.KeyboardBaseKeySize]
  );

  const baseKeyUnit = "px";
  return (
    <Flex gap="2" direction={"row"} justify={"center"}>
      {keys.map((key) => (
        <KeyFaceContainer
          keySize={1}
          baseKeySize={getBaseKeySize}
          baseKeyUnit={baseKeyUnit}
          hover={false}
          press={false}
          longPress={false}
          key={key}
        >
          <KeyFace shift={false} value={key.toUpperCase()} valueAlt={key} />
        </KeyFaceContainer>
      ))}
    </Flex>
  );
};
