import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import Text from "@src/components/Text";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";
import { observer } from "mobx-react-lite";
import TokenInput from "@components/TokenInput/TokenInput";
import BN from "@src/utils/BN";
import useCollapse from "@components/Collapse";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  padding: 20px;
  background-color: ${({ theme }) => theme.colors.dashboard.cardBackground};
  border-radius: 4px;
  width: 100%;
`;

const InputCard: React.FC<IProps> = () => {
  const vm = useDashboardVM();

  const handleMaxClick = () => {
    vm.onMaxBtnClick();
  };
  const handleCancelClick = () => {
    vm.setAction(null);
    vm.setTokenAmount(null);
    vm.setActionTokenAssetId(null);
  };

  const { getCollapseProps } = useCollapse({
    isExpanded: vm.action != null,
    duration: 500,
  });
  return (
    <Root {...getCollapseProps()}>
      <Text fitContent weight={600} type="secondary" size="small">
        {vm.operationName} {vm.actionToken.symbol}
      </Text>
      <SizedBox height={16} />
      <TokenInput
        decimals={vm.actionToken.decimals}
        amount={vm.tokenAmount ?? BN.ZERO}
        setAmount={vm.setTokenAmount}
        assetId={vm.actionToken.assetId}
        onMaxClick={handleMaxClick}
        balance={vm.tokenInputBalance}
      />
      <SizedBox height={8} />
      {vm.loading ? (
        <Button fixed>Loading...</Button>
      ) : (
        <Row>
          <Button kind="secondary" fixed onClick={handleCancelClick}>
            Cancel
          </Button>
          <SizedBox width={8} />
          <Button fixed onClick={vm.marketAction}>
            {vm.operationName}
          </Button>
        </Row>
      )}
    </Root>
  );
};
export default observer(InputCard);
