import SvgContainer, { SvgProps } from "../SvgContainer";

const ArrowLeft = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 25 50 L 75 30 V 70 Z" />
  </SvgContainer>
);

export default ArrowLeft;
