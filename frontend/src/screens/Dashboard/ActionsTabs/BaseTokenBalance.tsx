import styled from "@emotion/styled";
import React from "react";
import Card from "@components/Card";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import { Row } from "@src/components/Flex";
import { useStores } from "@stores";
import { observer } from "mobx-react-lite";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import Button from "@components/Button";
import Skeleton from "react-loading-skeleton";

interface IProps {}

const Root = styled.div`
  margin-bottom: 24px;
`;

const BaseTokenBalance: React.FC<IProps> = () => {
  const { accountStore } = useStores();

  const value = accountStore.getFormattedBalance(TOKENS_BY_SYMBOL.USDC);
  return (
    <Root>
      <Card>
        <Text type="secondary" weight={600} size="small">
          Usdc wallet balance
        </Text>
        <SizedBox height={4} />
        {value != null ? (
          <Text weight={600} size="big">
            {value}
          </Text>
        ) : (
          <Skeleton height={48} />
        )}
      </Card>
      <SizedBox height={20} />
      <Row>
        <Button fixed>Withdraw USDC</Button>
        <SizedBox width={10} />
        <Button fixed>Repay USDC</Button>
      </Row>
    </Root>
  );
};
export default observer(BaseTokenBalance);
