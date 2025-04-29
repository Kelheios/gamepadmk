export interface SvgProps {
  color?: string;
  fontFamily?: string;
  size?: string;
  thickness?: number;
}

export const Circle = () => <circle cx="50" cy="50" r="40" />;

export const defaultProps = {
  color: "currentColor",
  fontFamily: "Arial, sans-serif",
  size: "1rem",
  thickness: 4,
};

const SvgContainer = ({
  children,
  color = defaultProps.color,
  fontFamily = defaultProps.fontFamily,
  size = defaultProps.size,
  thickness = defaultProps.thickness,
}: {
  children: React.ReactNode;
  color?: string;
  fontFamily?: string;
  size?: string;
  thickness?: number;
}) => {
  return (
    <svg
      style={{ width: size, height: size }}
      fill="none"
      stroke={color}
      strokeWidth={thickness}
      fontFamily={fontFamily}
      viewBox="0 0 100 100"
      strokeLinecap="round"
      strokeLinejoin="round"
      xmlns="http://www.w3.org/2000/svg"
    >
      {children}
    </svg>
  );
};

export default SvgContainer;
