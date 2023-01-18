import styled from "@emotion/styled";
import React from "react";
import ModeSwitch from "@screens/Dashboard/ModeSwitch";
import { TAction, useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import SizedBox from "@components/SizedBox";
import SummaryCard from "./SummaryCard";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import ActionTab from "@screens/Dashboard/ActionsTabs/ActionTab";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;

const RightBlock: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const handleUsdcClick = (action: TAction) => {
    vm.setAction(action);
    vm.setActionTokenAssetId(TOKENS_BY_SYMBOL.USDC.assetId);
  };
  //0 - deposit
  //1 - borrow usdc
  return (
    <Root>
      <ModeSwitch />
      <SizedBox height={20} />

      {vm.actionTokenAssetId == null ? (
        vm.mode === 0 ? (
          <Row>
            <Button fixed onClick={() => handleUsdcClick("supply")}>
              Supply USDC
            </Button>
            <SizedBox width={10} />
            <Button fixed onClick={() => handleUsdcClick("withdraw")}>
              Withdraw USDC
            </Button>
          </Row>
        ) : (
          <Row>
            <Button fixed onClick={() => handleUsdcClick("borrow")}>
              Borrow USDC
            </Button>
            <SizedBox width={10} />
            <Button fixed onClick={() => handleUsdcClick("repay")}>
              Repay USDC
            </Button>
          </Row>
        )
      ) : (
        <ActionTab />
      )}
      <SizedBox height={24} />
      <SummaryCard />
    </Root>
  );
};
export default observer(RightBlock);
