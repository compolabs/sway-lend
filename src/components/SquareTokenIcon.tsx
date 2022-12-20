import styled from "@emotion/styled";

type TokenIconSize = "default" | "small";

const SquareTokenIcon = styled.img<{ size?: TokenIconSize }>`
  border: 1px solid ${({ theme }) => `${theme.colors.icon.borderColor}`};
  border-radius: ${({ size }) => (size === "small" ? "8px" : "12px")};
  box-sizing: border-box;
  box-shadow: none;
  color: transparent;
  object-fit: cover;
  ${({ size }) =>
    (() => {
      switch (size) {
        case "small":
          return "width: 40px; height: 40px;";
        default:
          return "width: 56px; height: 56px;";
      }
    })()}
`;

export default SquareTokenIcon;
