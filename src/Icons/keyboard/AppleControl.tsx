import SvgContainer, { SvgProps } from "../SvgContainer";

const AppleControl = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 25 45 L 50 20 L 75 45" />
  </SvgContainer>
);

export default AppleControl;
