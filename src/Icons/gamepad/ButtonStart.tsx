import SvgContainer, { Circle, SvgProps } from "../SvgContainer";

const ButtonStart = (props: SvgProps) => (
  <SvgContainer {...props}>
    <Circle />
    <path d="M30 36l40 0" />
    <path d="M30 50l40 0" />
    <path d="M30 64l40 0" />
  </SvgContainer>
);

export default ButtonStart;
