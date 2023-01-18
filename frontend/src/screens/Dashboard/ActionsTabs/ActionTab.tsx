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
import toUpperFirstLetter from "@src/utils/toUpperFirstLetter";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  @media (min-width: 880px) {
    margin-top: 0;
  }
`;
const Card = styled.div`
  display: flex;
  flex-direction: column;
  padding: 20px;
  background-color: ${({ theme }) => theme.colors.dashboard.cardBackground};
  border-radius: 4px;
  margin-bottom: 24px;
`;

const ActionTab: React.FC<IProps> = () => {
  const vm = useDashboardVM();

  const handleCancelClick = () => {
    vm.setAction(null);
    vm.setActionTokenAssetId(null);
  };

  const { getCollapseProps } = useCollapse({
    isExpanded: vm.action != null,
    duration: 500,
  });
  return (
    <Root>
      <Card {...getCollapseProps()}>
        <Text fitContent weight={600} type="secondary" size="small">
          {toUpperFirstLetter(vm.action ?? "")} {vm.token.symbol}
        </Text>
        <SizedBox height={16} />
        <TokenInput
          decimals={vm.token.decimals}
          amount={vm.tokenAmount ?? BN.ZERO}
          setAmount={vm.setTokenAmount}
          assetId={vm.token.assetId}
        />
        <SizedBox height={8} />
        <Row>
          <Button kind="secondary" fixed onClick={handleCancelClick}>
            Cancel
          </Button>
          <SizedBox width={8} />
          <Button fixed>{toUpperFirstLetter(vm.action ?? "")}</Button>
        </Row>
      </Card>
    </Root>
  );
};
export default observer(ActionTab);
