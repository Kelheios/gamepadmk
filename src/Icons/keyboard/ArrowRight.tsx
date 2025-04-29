import SvgContainer, { SvgProps } from "../SvgContainer";

const ArrowRight = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 75 50 L 25 70 V 30 Z" />
  </SvgContainer>
);

export default ArrowRight;
