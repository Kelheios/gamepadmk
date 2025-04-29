import defaultLayoutLinux from "./default/linux.json";
import defaultLayoutMacOs from "./default/macos.json";
import defaultLayoutWindows from "./default/windows.json";

import { isLinux, isMacOs, isWindows } from "../../utils/os";

type VirtualKey = {
  value: string;
  valueAlt?: string;
  label?: string;
  icon?: string;
  iconPosition?: "left" | "right" | "top" | "bottom";
  size: number;
  offsetLeft?: number;
  offsetRight?: number;
};

type KeyLayout = VirtualKey[][];

const getDefaultLayout = () =>
  isLinux
    ? defaultLayoutLinux
    : isMacOs
    ? defaultLayoutMacOs
    : isWindows
    ? defaultLayoutWindows
    : defaultLayoutWindows;

const defineKey = ({
  value,
  valueAlt,
  label,
  icon,
  iconPosition,
  size,
  offsetLeft,
  offsetRight,
}: VirtualKey): VirtualKey => ({
  value,
  valueAlt,
  icon,
  iconPosition,
  label,
  size,
  offsetLeft,
  offsetRight,
});

const parseLayoutJson = (json: KeyLayout): KeyLayout =>
  json.map((row) => row.map((key) => defineKey(key)));

const layout = parseLayoutJson(getDefaultLayout() as KeyLayout);

export { layout, type KeyLayout };
