import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import Text from "@components/Text";
// import info from "@src/assets/icons/info.svg";
import error from "@src/assets/icons/error.svg";
import warning from "@src/assets/icons/warning.svg";
import SizedBox from "@components/SizedBox";
import { Column } from "./Flex";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  type: "warning" | "info" | "error";
  text: string | JSX.Element;
}

const Root = styled.div<{ type: "warning" | "info" | "error" }>`
  display: flex;
  flex-direction: row;
  ${({ type, theme }) =>
    (() => {
      switch (type) {
        case "warning":
          return `background: ${theme.colors.attention100};`;
        case "info":
          return `background: ${theme.colors.primary100};`;
        case "error":
          return `background: ${theme.colors.error100};`;
        default:
          return `background: ${theme.colors.primary100};`;
      }
    })()}
  border-radius: 12px;
  padding: 18px;
  justify-content: flex-start;

  a {
    color: #6563dd;
    text-decoration: underline;
  }
`;

const Notification: React.FC<IProps> = ({ text, type, ...rest }) => {
  const getIcon = () => {
    switch (type) {
      case "error":
        return error;
      case "info":
        // return info;
        return warning;
      case "warning":
        return warning;
    }
  };
  return (
    <Root type={type} {...rest}>
      <Column>
        <img src={getIcon()} alt="info" />
      </Column>
      <SizedBox width={10} />
      <Column justifyContent="center" mainAxisSize="stretch">
        {typeof text === "string" ? (
          <Text style={{ height: "100%" }} size="medium">
            {text}
          </Text>
        ) : (
          text
        )}
      </Column>
    </Root>
  );
};
export default Notification;
