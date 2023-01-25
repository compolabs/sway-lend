import styled from "@emotion/styled";
import React from "react";
import { ACTION_TYPE, useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import SizedBox from "@components/SizedBox";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import { useStores } from "@stores";
import InputCard from "@screens/Dashboard/ActionTab/InputCard";
import useCollapse from "@src/components/Collapse";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
`;

const ActionTab: React.FC<IProps> = () => {
  const { accountStore, settingsStore } = useStores();
  const vm = useDashboardVM();
  const handleBaseTokenClick = (action: ACTION_TYPE) => {
    vm.setAction(action);
    vm.setTokenAmount(null);
    vm.setActionTokenAssetId(TOKENS_BY_SYMBOL.USDC.assetId);
  };
  const { getCollapseProps } = useCollapse({
    isExpanded: vm.action == null,
    duration: 300,
  });
  return (
    <Root>
      {!accountStore.isLoggedIn && (
        <>
          <Button fixed onClick={() => settingsStore.setLoginModalOpened(true)}>
            Connect wallet
          </Button>
          <SizedBox height={10} />
        </>
      )}
      <div {...getCollapseProps()}>
        {vm.mode === 0 ? (
          <Row>
            <Button
              fixed
              onClick={() => handleBaseTokenClick(ACTION_TYPE.SUPPLY)}
              disabled={!accountStore.isLoggedIn}
            >
              Supply {vm.baseToken.symbol}
            </Button>
            <SizedBox width={10} />
            <Button
              fixed
              onClick={() => handleBaseTokenClick(ACTION_TYPE.WITHDRAW)}
              disabled={!accountStore.isLoggedIn || vm.suppliedBalance?.eq(0)}
            >
              Withdraw {vm.baseToken.symbol}
            </Button>
          </Row>
        ) : (
          <Row>
            <Button
              fixed
              onClick={() => handleBaseTokenClick(ACTION_TYPE.BORROW)}
              disabled={
                !accountStore.isLoggedIn || vm.maxBorrowBaseTokenAmount?.eq(0)
              }
            >
              Borrow {vm.baseToken.symbol}
            </Button>
            <SizedBox width={10} />
            <Button
              fixed
              onClick={() => handleBaseTokenClick(ACTION_TYPE.REPAY)}
              disabled={!accountStore.isLoggedIn || vm.borrowedBalance?.eq(0)}
            >
              Repay {vm.baseToken.symbol}
            </Button>
          </Row>
        )}
      </div>
      <InputCard />
    </Root>
  );
};
export default observer(ActionTab);
