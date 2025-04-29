import SvgContainer, { Circle, SvgProps } from "../SvgContainer";

const CirclePlus = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <path
      d="M 30 50 L 70 50 M 50 30 L 50 70"
      stroke={props.color}
      strokeLinecap="round"
    />
  </SvgContainer>
);

export default CirclePlus;
