import SvgContainer, { Circle, SvgProps } from "../SvgContainer";

const CircleMinus = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <path d="M 30 50 L 70 50" stroke={props.color} strokeLinecap="round" />
  </SvgContainer>
);

export default CircleMinus;
