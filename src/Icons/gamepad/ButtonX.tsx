import SvgContainer, { Circle, defaultProps, SvgProps } from "../SvgContainer";

const ButtonX = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <text
      dominantBaseline="middle"
      fill={props.color}
      fontSize="48"
      strokeWidth={(props.thickness ?? defaultProps.thickness) * 0.5}
      textAnchor="middle"
      x="50"
      y="54"
    >
      X
    </text>
  </SvgContainer>
);

export default ButtonX;
