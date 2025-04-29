import SvgContainer, { SvgProps } from "../SvgContainer";

const ArrowUp = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 50 25 L 30 75 H 70 Z" />
  </SvgContainer>
);

export default ArrowUp;
