import { type } from "@tauri-apps/plugin-os";

const OS_TYPE_IS = type();

enum OsType {
  MacOS = "macos",
  Windows = "windows",
  Linux = "linux",
}

const isLinux = OS_TYPE_IS === OsType.Linux;
const isMacOs = OS_TYPE_IS === OsType.MacOS;
const isWindows = OS_TYPE_IS === OsType.Windows;

export { OS_TYPE_IS, isMacOs, isWindows, isLinux, OsType };
