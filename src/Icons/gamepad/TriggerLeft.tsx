import SvgContainer, { defaultProps, SvgProps } from "../SvgContainer";

const TriggerLeft = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 85 74 H 40 A 40 40 0 0 1 15 50 V 26 H 85 Z" fill="none" />
    <text
      dominantBaseline="bottom"
      fill={props.color}
      fontFamily="Arial, Helvetica, sans-serif"
      fontSize="36"
      strokeWidth={(props.thickness ?? defaultProps.thickness) * 0.5}
      textAnchor="middle"
      x="56"
      y="64"
    >
      LT
    </text>
  </SvgContainer>
);

export default TriggerLeft;
