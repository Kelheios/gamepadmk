import { Box } from "@radix-ui/themes";

import "./animation.css";

export const KeyFaceContainer = ({
  baseKeySize,
  baseKeyUnit,
  keySize,
  keyOffsetLeft,
  keyOffsetRight,
  hover,
  press,
  longPress,
  children,
}: {
  baseKeySize: number;
  baseKeyUnit: string;
  keySize: number;
  keyOffsetLeft?: number;
  keyOffsetRight?: number;
  hover: boolean;
  press: boolean;
  longPress: boolean;
  children: React.ReactNode;
}) => {
  const HOVER_COLOR = "red";
  const HOVER_DURATION = 0.125;
  const PRESS_COLOR = "red";
  const PRESS_SIZE = 5;
  const PRESS_DURATION = 0.25;
  const LONG_PRESS_ANIMATION = "pulse 1s infinite";
  const LONG_PRESS_TRANSFORM = "scale(1)";

  return (
    <Box
      className="keyFrame"
      style={{
        transform: longPress ? LONG_PRESS_TRANSFORM : "",
        animation: longPress ? LONG_PRESS_ANIMATION : "",
        width: `${keySize * baseKeySize}${baseKeyUnit}`,
        height: `${baseKeySize}${baseKeyUnit}`,
        marginLeft: `${(keyOffsetLeft ?? 0) * baseKeySize}${baseKeyUnit}`,
        marginRight: `${(keyOffsetRight ?? 0) * baseKeySize}${baseKeyUnit}`,
        borderWidth: "1px",
        borderStyle: "solid",
        background: hover ? HOVER_COLOR : "inherit",
        boxShadow: press
          ? `0 0 ${PRESS_SIZE}px ${PRESS_SIZE}px ${PRESS_COLOR}`
          : "none",
        transition: `background ${HOVER_DURATION}s ease, box-shadow ${PRESS_DURATION}s ease`,
      }}
    >
      {children}
    </Box>
  );
};
