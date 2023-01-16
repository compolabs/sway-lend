import styled from "@emotion/styled";

type TButtonType = "primary" | "secondary" | "danger";
//todo
type TButtonSize = "medium" | "large";

const Button = styled.button<{
  kind?: TButtonType;
  size?: TButtonSize;
  fixed?: boolean;
}>`
  white-space: nowrap;
  display: flex;
  justify-content: center;
  align-items: center;
  box-sizing: border-box;

  border: 1px solid
    ${({ kind, theme }) =>
      (() => {
        switch (kind) {
          case "primary":
            return theme.colors.primary01;
          case "secondary":
            return theme.colors.primary100;
          case "danger":
            return theme.colors.error500;
          default:
            return theme.colors.primary01;
        }
      })()};

  border-radius: 4px;
  box-shadow: none;
  font-style: normal;
  font-weight: 700;
  font-size: 13px;
  line-height: 24px;

  width: ${({ fixed }) => (fixed ? "100%" : "fit-content")};
  transition: 0.4s;

  ${({ kind, theme }) =>
    (() => {
      switch (kind) {
        case "primary":
          return `background: ${theme.colors.primary01}; color:${theme.colors.neutral7};`;
        case "secondary":
          return `background: ${theme.colors.white}; color:${theme.colors.blue500};`;
        case "danger":
          return `background: ${theme.colors.error500}; color: #fff;`;
        default:
          return `background: ${theme.colors.primary01}; color:${theme.colors.neutral7};`;
      }
    })()}
  ${({ size }) =>
    (() => {
      switch (size) {
        case "medium":
          return "padding: 0 20px; height: 40px;";
        default:
          return "padding: 0 20px; height: 40px;";
      }
    })()}
  :hover {
    cursor: pointer;
    ${({ kind }) =>
      (() => {
        switch (kind) {
          case "primary":
            return "opacity: 0.8;";
          case "secondary":
            return "opacity: 0.8;";
          case "danger":
            return "opacity: 0.8;";
          default:
            return "opacity: 0.8;";
        }
      })()}
  }

  :disabled {
    ${({ kind, theme }) =>
      (() => {
        switch (kind) {
          case "primary":
            return `background: ${theme.colors.disabledBtnColor}; border: none; color:${theme.colors.disabledBtnTextColor}; `;
          case "secondary":
            return `background: ${theme.colors.white}; border: 1px solid ${theme.colors.primary100};`;
          case "danger":
            return `background: ${theme.colors.error100}; border: 1px solid ${theme.colors.error100};`;
          default:
            return `background: ${theme.colors.disabledBtnColor}; border:none; color:${theme.colors.disabledBtnTextColor}; `;
        }
      })()}
    cursor: not-allowed;
  }
`;

export default Button;
