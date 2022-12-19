import styled from "@emotion/styled";

const Card = styled.div<{
  maxWidth?: number;
  paddingDesktop?: string;
  paddingMobile?: string;
  justifyContent?:
    | "start"
    | "flex-end"
    | "space-around"
    | "space-between"
    | "center";
  alignItems?:
    | "start"
    | "end"
    | "center"
    | "inherit"
    | "unset"
    | "flex-end"
    | "flex-start"
    | "baseline";
  flexDirection?: "column" | "row";
  type?: "white" | "dark";
  bordered?: boolean;
}>`
  display: flex;
  flex-direction: ${({ flexDirection }) => flexDirection ?? "column"};
  justify-content: ${({ justifyContent }) => justifyContent ?? "default"};
  align-items: ${({ alignItems }) => alignItems ?? "default"};
  max-width: ${({ maxWidth }) => `${maxWidth}px` ?? "100%"};
  ${({ bordered, theme }) =>
    bordered && `border: 1px solid ${theme.colors.primary100};`};
  width: 100%;
  border: 1px solid ${({ theme }) => `${theme.colors.primary100}`};
  border-radius: 16px;
  box-sizing: border-box;
  padding: ${({ paddingMobile }) => paddingMobile ?? "16px"};
  ${({ type, theme }) =>
    (() => {
      switch (type) {
        case "white":
          return `background: ${theme.colors.card.background};`;
        case "dark":
          return `background: ${theme.colors.blue500};`;
        default:
          return `background: ${theme.colors.card.background};`;
      }
    })()};
  @media (min-width: 560px) {
    padding: ${({ paddingDesktop }) => paddingDesktop ?? "24px"};
  }
`;
export default Card;
