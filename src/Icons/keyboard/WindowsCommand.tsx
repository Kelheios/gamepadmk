import SvgContainer, { SvgProps } from "../SvgContainer";

const WindowsCommand = (props: SvgProps) => (
  <SvgContainer {...props}>
    <g fill={props.color}>
      <rect x="20" y="20" width="25" height="25" rx="1" ry="1" />
      <rect x="55" y="20" width="25" height="25" rx="1" ry="1" />
      <rect x="20" y="55" width="25" height="25" rx="1" ry="1" />
      <rect x="55" y="55" width="25" height="25" rx="1" ry="1" />
    </g>
  </SvgContainer>
);

export default WindowsCommand;
