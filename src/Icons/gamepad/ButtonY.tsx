import SvgContainer, { Circle, defaultProps, SvgProps } from "../SvgContainer";

const ButtonY = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <text
      dominantBaseline="middle"
      fill={props.color}
      fontSize="48"
      strokeWidth={(props.thickness ?? defaultProps.thickness) * 0.5}
      textAnchor="middle"
      x="51"
      y="54"
    >
      Y
    </text>
  </SvgContainer>
);

export default ButtonY;
