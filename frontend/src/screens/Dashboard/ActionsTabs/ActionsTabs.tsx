import styled from "@emotion/styled";
import React from "react";
import ActionTab from "./ActionTab";
import ModeSwitch from "@screens/Dashboard/ModeSwitch";
import SizedBox from "@components/SizedBox";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;

const ActionsTabs: React.FC<IProps> = () => {
  return (
    <Root>
      <ModeSwitch />
      <SizedBox height={24} />
      <ActionTab />
    </Root>
  );
};
export default ActionsTabs;
