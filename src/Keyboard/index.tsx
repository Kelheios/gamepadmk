import { useEffect, useState } from "react";
import { emit } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/dpi";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import { KeyLayout, layout } from "./layouts";

import { Box, Flex } from "@radix-ui/themes";

import useSettings, { SettingDefault, SettingKey } from "../hooks/use-settings";
import { KeyFaceContainer } from "./KeyFaceContainer";
import { KeyFace } from "./KeyFace";

interface KeyboardNavigationPosition {
  x: number;
  y: number;
}

const getKeyAtPosition = (
  layout: KeyLayout,
  { x, y }: KeyboardNavigationPosition
) => layout[y][x].value;

const getAltKeyAtPosition = (
  layout: KeyLayout,
  { x, y }: KeyboardNavigationPosition
) => layout[y][x].valueAlt;

export default function Keyboard() {
  const [keyboardNavigationPosition, setKeyboardNavigationPosition] =
    useState<KeyboardNavigationPosition>({
      x: 0,
      y: 0,
    });

  const [keyboardShiftState, setKeyboardShiftState] = useState<boolean>(false);
  useEffect(() => {
    emit("KeyboardSetActiveKey", {
      key: keyboardShiftState
        ? getAltKeyAtPosition(layout, keyboardNavigationPosition)
        : getKeyAtPosition(layout, keyboardNavigationPosition),
    });
  }, [keyboardNavigationPosition, keyboardShiftState]);

  useEffect(() => {
    const appWebview = getCurrentWebviewWindow();
    const unlistenKeyboardNavigationIntention = appWebview.listen<string>(
      "KeyboardNavigationIntention",
      ({ payload }) => {
        const navigationIntent = JSON.parse(payload);
        setKeyboardNavigationPosition((prev) => {
          const nextY = Math.max(
            0,
            Math.min(layout.length - 1, prev.y + navigationIntent.y)
          );

          const nextX = Math.max(
            0,
            Math.min(layout[nextY].length - 1, prev.x + navigationIntent.x)
          );
          return {
            x: nextX,
            y: nextY,
          };
        });
      }
    );
    const unlistenKeyboardShiftIntention = appWebview.listen<string>(
      "KeyboardShiftIntention",
      ({ payload }) => {
        const shiftIntent = JSON.parse(payload);
        setKeyboardShiftState(shiftIntent);
      }
    );
    return () => {
      unlistenKeyboardNavigationIntention.then((f) => f());
      unlistenKeyboardShiftIntention.then((f) => f());
    };
  }, []);

  const [heldPosition, setHeldPosition] =
    useState<KeyboardNavigationPosition | null>(null);

  useEffect(() => {
    const appWebview = getCurrentWebviewWindow();
    const unlistenKeyboardKeyPress = appWebview.listen<string>(
      "KeyboardPressIntention",
      ({ payload }) => {
        const pressIntent = JSON.parse(payload);
        if (pressIntent) {
          setHeldPosition(keyboardNavigationPosition);
        } else {
          setHeldPosition(null);
        }
      }
    );
    return () => {
      unlistenKeyboardKeyPress.then((f) => f());
    };
  }, [keyboardNavigationPosition]);

  const isCurretNavigationPosition = (x: number, y: number) =>
    x === keyboardNavigationPosition.x && y === keyboardNavigationPosition.y;

  const isCurrentHeldPosition = (x: number, y: number) =>
    heldPosition?.x === x && heldPosition?.y === y;

  const [longPressPositions, setLongPressPositions] = useState<
    KeyboardNavigationPosition[]
  >([]);

  useEffect(() => {
    const appWebview = getCurrentWebviewWindow();
    const unlistenKeyboardKeyPress = appWebview.listen<string>(
      "KeyboardLongPressIntention",
      ({ payload }) => {
        const longPressIntent = JSON.parse(payload);
        if (longPressIntent) {
          setLongPressPositions((prev) => [
            ...prev,
            keyboardNavigationPosition,
          ]);
        } else {
          setLongPressPositions((prev) =>
            prev.filter(
              (position) =>
                position.x !== keyboardNavigationPosition.x ||
                position.y !== keyboardNavigationPosition.y
            )
          );
        }
      }
    );
    return () => {
      unlistenKeyboardKeyPress.then((f) => f());
    };
  }, [keyboardNavigationPosition]);

  useEffect(() => {
    const appWebview = getCurrentWebviewWindow();
    const unlistenKeyboardLongPressClearAll = appWebview.listen<string>(
      "KeyboardLongPressClearAllIntention",
      ({ payload }) => {
        const longPressClearAllIntent = JSON.parse(payload);
        if (longPressClearAllIntent) {
          setLongPressPositions([]);
        }
      }
    );
    return () => {
      unlistenKeyboardLongPressClearAll.then((f) => f());
    };
  }, []);

  const isCurrentLongHeldKeyPosition = (x: number, y: number) =>
    longPressPositions.some((position) => position.x === x && position.y === y);

  const [keyboard] = useSettings<boolean>(
    SettingKey.Keyboard,
    SettingDefault[SettingKey.Keyboard]
  );

  const [baseKeySize] = useSettings<number>(
    SettingKey.KeyboardBaseKeySize,
    SettingDefault[SettingKey.KeyboardBaseKeySize]
  );
  const baseKeyUnit = "px";
  const keyboardCols = 15;
  const keyboardRows = 6;

  useEffect(() => {
    if (!keyboard) {
      getCurrentWebviewWindow().hide();
      return;
    }
    const size = new LogicalSize(
      baseKeySize * keyboardCols,
      baseKeySize * keyboardRows
    );
    getCurrentWebviewWindow().setSize(size);
    moveWindow(Position.BottomCenter);
    getCurrentWebviewWindow().show();
  }, [baseKeySize, keyboard]);

  return (
    <Box>
      <Flex direction={"column"}>
        {layout.map((row, yIndex) => (
          <Flex justify={"between"} key={yIndex}>
            {row.map((key, xIndex) => (
              <KeyFaceContainer
                key={`${xIndex}-${yIndex}`}
                baseKeySize={baseKeySize}
                baseKeyUnit={baseKeyUnit}
                keySize={key.size}
                keyOffsetLeft={key.offsetLeft}
                keyOffsetRight={key.offsetRight}
                hover={isCurretNavigationPosition(xIndex, yIndex)}
                press={isCurrentHeldPosition(xIndex, yIndex)}
                longPress={isCurrentLongHeldKeyPosition(xIndex, yIndex)}
              >
                <KeyFace
                  value={key.value}
                  valueAlt={key.valueAlt}
                  shift={keyboardShiftState}
                  label={key.label}
                  icon={key.icon}
                  iconPosition={key.iconPosition}
                />
              </KeyFaceContainer>
            ))}
          </Flex>
        ))}
      </Flex>
    </Box>
  );
}

export { KeyFace, KeyFaceContainer };
