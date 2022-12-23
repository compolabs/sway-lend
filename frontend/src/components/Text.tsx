import styled from "@emotion/styled";

type TTextType =
  | "primary"
  | "secondary"
  | "light"
  | "blue500"
  | "error"
  | "success"
  | "purple300";
type TTextSize = "small" | "medium" | "large" | "big";
type TTextAlign = "center" | "left" | "right" | "justify" | "end";

const Text = styled.p<{
  type?: TTextType;
  weight?: 400 | 500;
  size?: TTextSize;
  fitContent?: boolean;
  nowrap?: boolean;
  crossed?: boolean;
  ellipsis?: number;
  textAlign?: TTextAlign;
}>`
  margin: 0;
  width: ${({ fitContent }) => (fitContent ? "fit-content" : "100%")};
  font-weight: ${({ weight }) => weight ?? 400};
  white-space: ${({ nowrap }) => (nowrap ? "nowrap" : "unset")};
  text-decoration: ${({ crossed }) => (crossed ? "line-through" : "unset")};
  text-align: ${({ textAlign }) => textAlign ?? "default"};
  ${({ type, theme }) =>
    (() => {
      switch (type) {
        case "primary":
          return `color: ${theme.colors?.primary800};`;
        case "secondary":
          return `color: ${theme.colors?.primary650};`;
        case "blue500":
          return `color: ${theme.colors?.blue500};`;
        case "light":
          return `color: ${theme.colors?.white};`;
        case "error":
          return `color: ${theme.colors?.error500};`;
        case "success":
          return `color: ${theme.colors?.success};`;
        case "purple300":
          return `color: ${theme.colors?.primary300};`;
        default:
          return `color: ${theme.colors?.primary800};`;
      }
    })()}
  ${({ ellipsis }) =>
    ellipsis != null &&
    `max-width: ${ellipsis}px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;`};
  ${({ size }) =>
    (() => {
      switch (size) {
        case "small":
          return "font-size: 12px; line-height: 16px;";
        case "medium":
          return "font-size: 14px; line-height: 20px;";
        case "big":
          return "font-size: 20px; line-height: 24px;";
        case "large":
          return "font-size: 32px;line-height: 40px;";
        default:
          return "font-size: 16px; line-height: 24px;";
      }
    })()}
`;

export default Text;
