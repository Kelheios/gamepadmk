import SvgContainer, { SvgProps } from "../SvgContainer";

const ArrowDown = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 50 75 L 30 25 L 70 25 Z" />
  </SvgContainer>
);

export default ArrowDown;
