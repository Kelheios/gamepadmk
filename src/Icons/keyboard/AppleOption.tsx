import SvgContainer, { SvgProps } from "../SvgContainer";

const AppleOption = (props: SvgProps) => (
  <SvgContainer {...props}>
    <path d="M 20 25 L 40 25 L 60 75 L 80 75" />
    <path d="M 60 25 L 80 25" />
  </SvgContainer>
);

export default AppleOption;
