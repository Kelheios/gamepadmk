import SvgContainer, { Circle, SvgProps } from "../SvgContainer";

const ButtonBack = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <path d="M 35 53 L 30 53 L 30 31 L 60 31 L 60 40" />
    <rect x="40" y="45" width="30" height="22" fill="transparent" />
  </SvgContainer>
);

export default ButtonBack;
