import styled from "@emotion/styled";
import React from "react";
import plus from "@src/assets/icons/plus.svg";
import minus from "@src/assets/icons/minus.svg";
import { useTheme } from "@emotion/react";

interface IProps {
  selected?: boolean;
  type: "minus" | "plus";
}

const Root = styled.div<{ selected?: boolean }>`
  display: flex;
  flex-direction: column;
  width: 42px;
  height: 42px;
  border-radius: 50%;
  align-items: center;
  justify-content: center;

  background-color: ${({ theme, selected }) =>
    selected
      ? theme.colors.supplyBtn.backgroundSelected
      : theme.colors.supplyBtn.background};
  opacity: ${({ selected }) => selected && "0.5"};
`;

const Symbol: React.FC<IProps> = ({ selected, type }) => {
  const theme = useTheme();
  return (
    <Root selected={selected}>
      <img
        style={{ width: 8, height: 8 }}
        src={
          type === "minus" ? theme.images.icons.minus : theme.images.icons.plus
        }
        alt="symbol"
      />
    </Root>
  );
};
export default Symbol;
