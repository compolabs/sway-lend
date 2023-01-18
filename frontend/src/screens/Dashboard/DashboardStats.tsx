import styled from "@emotion/styled";
import React from "react";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import { Column, Row } from "@src/components/Flex";
import Text from "@src/components/Text";

interface IProps {}

const Root = styled.div`
  width: 100%;
  display: grid;
  align-items: flex-end;
  @media (min-width: 880px) {
    grid-template-columns: 2fr 1fr;
  }
`;

const DashboardStats: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  return (
    <Root>
      <Row alignItems="end" justifyContent="space-between">
        <Column crossAxisSize="max">
          <Text size="medium" fitContent type="secondary" weight={600}>
            Supply balance
          </Text>
          <Text size="large" fitContent weight={600}>
            $5,532.32
          </Text>
        </Column>
        <Column crossAxisSize="max">
          <Text size="medium" fitContent type="secondary" weight={600}>
            APY
          </Text>
          <Text size="big" fitContent weight={600}>
            32.54%
          </Text>
        </Column>
      </Row>
      <Row justifyContent="space-between">
        <Column crossAxisSize="max">
          <Text textAlign="end" size="medium" type="secondary" weight={600}>
            Borrow balance
          </Text>
          <Text textAlign="end" size="big" weight={600}>
            $1,000,514.63
          </Text>
        </Column>
      </Row>
    </Root>
  );
};
export default observer(DashboardStats);
