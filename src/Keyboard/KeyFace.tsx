import {
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,
  WindowsCommand,
  AppleControl,
  AppleOption,
  TriggerLeft,
  AppleCommand,
} from "../Icons";

const KeyIcon = ({
  icon,
  color = "currentColor",
  size = "1rem",
}: {
  icon: string;
  color?: string;
  size?: string;
}) => {
  const AVAILABLE_ICONS = {
    ARROW_LEFT: ArrowLeft,
    ARROW_RIGHT: ArrowRight,
    ARROW_UP: ArrowUp,
    ARROW_DOWN: ArrowDown,
    CONTROL: AppleControl,
    OPTION: AppleOption,
    TRIGGER_LEFT: TriggerLeft,
    APPLE: AppleCommand,
    WINDOWS: WindowsCommand,
  };

  if (!AVAILABLE_ICONS[icon as keyof typeof AVAILABLE_ICONS]) {
    return null;
  }

  const Icon = AVAILABLE_ICONS[icon as keyof typeof AVAILABLE_ICONS];
  return <Icon size={size} color={color} thickness={8} />;
};

export const KeyFace = ({
  shift,
  value,
  valueAlt,
  icon,
  iconPosition,
  label,
}: {
  shift: boolean;
  value?: string;
  valueAlt?: string;
  icon?: string;
  iconPosition?: "left" | "right" | "top" | "bottom";
  label?: string;
}) => (
  <div style={{ height: "100%", width: "100%" }}>
    {valueAlt ? (
      <>
        <div style={{ position: "absolute", padding: 4, fontSize: "75%" }}>
          {!shift ? valueAlt : value}
        </div>
        <div
          style={{
            fontSize: "1rem",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            height: "100%",
            width: "100%",
          }}
        >
          {shift ? valueAlt : value}
        </div>
      </>
    ) : (
      <div
        style={{
          fontSize: "1rem",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          justifyItems: "center",
          height: "100%",
          width: "100%",
          flexDirection:
            iconPosition === "top" || iconPosition === "bottom"
              ? "column"
              : "row",
        }}
      >
        <span
          style={{
            order: 0,
          }}
        >
          {label ?? value}
        </span>
        <span
          style={{
            display: "flex",
            alignItems: "center",
            order:
              iconPosition === "left"
                ? -1
                : iconPosition === "right"
                ? 1
                : iconPosition === "top"
                ? -1
                : 1,
          }}
        >
          {icon && <KeyIcon icon={icon} size="1.25rem" />}
        </span>
      </div>
    )}
  </div>
);
