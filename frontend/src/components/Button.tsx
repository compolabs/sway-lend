import styled from "@emotion/styled";

type TButtonType = "primary" | "secondary";

const Button = styled.button<{
  kind?: TButtonType;
  fixed?: boolean;
}>`
  white-space: nowrap;
  display: flex;
  justify-content: center;
  align-items: center;
  box-sizing: border-box;

  border: none;
  border-radius: 4px;
  box-shadow: none;
  font-style: normal;
  font-weight: 700;
  font-size: 13px;
  line-height: 24px;

  width: ${({ fixed }) => (fixed ? "100%" : "fit-content")};
  padding: 8px 16px;

  color: ${({ theme, kind }) =>
    kind === "secondary"
      ? theme.colors.button.secondaryColor
      : theme.colors.button.primaryColor};

  background: ${({ theme, kind }) =>
    kind === "secondary"
      ? theme.colors.button.secondaryBackground
      : theme.colors.button.primaryBackground};

  :hover {
    background: ${({ theme, kind }) =>
      kind === "secondary"
        ? theme.colors.button.secondaryBackgroundHover
        : theme.colors.button.primaryBackgroundHover};
    cursor: pointer;
  }

  :disabled {
    background: ${({ theme }) => theme.colors.button.backgroundDisabled};
    color: ${({ theme }) => theme.colors.neutral4};
    opacity: 0.6;
    cursor: not-allowed;
  }
`;

export default Button;
