import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import { Column, Row } from "@src/components/Flex";
import Text from "@src/components/Text";
import SizedBox from "@components/SizedBox";
import Divider from "@src/components/Divider";
import useWindowSize from "@src/hooks/useWindowSize";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import BN from "@src/utils/BN";
import Skeleton from "react-loading-skeleton";

interface IProps {}

const Root = styled.div`
  width: 100%;
  display: grid;
  align-items: flex-end;

  .main-data {
    font-size: 48px;
    line-height: 48px;
  }

  .data {
    font-size: 24px;
    line-height: 32px;
  }

  .title {
    font-size: 14px;
    line-height: 24px;
  }

  @media (min-width: 880px) {
    grid-template-columns: 2fr 1fr;
    .main-data {
      font-size: 64px;
      line-height: 64px;
    }

    .data {
      font-size: 40px;
      line-height: 48px;
    }

    .title {
      font-size: 18px;
      line-height: 24px;
    }
  }
`;
const DashboardStats: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const { width } = useWindowSize();
  const supplied = BN.formatUnits(
    vm.suppliedBalance ?? BN.ZERO,
    vm.baseToken.decimals
  ).toFormat(2);

  const borrowed = BN.formatUnits(
    vm.borrowedBalance ?? BN.ZERO,
    vm.baseToken.decimals
  ).toFormat(2);

  return (
    <Root>
      {width && width >= 880 ? (
        !vm.initialized ? (
          <>
            <Column>
              <Skeleton height={24} width={200} />
              <Skeleton height={64} width={200} />
            </Column>
            <Column crossAxisSize="max" alignItems="end">
              <Skeleton height={24} width={200} />
              <Skeleton height={32} width={200} />
            </Column>
          </>
        ) : (
          <>
            <Row justifyContent="space-between" alignItems="end">
              <Column crossAxisSize="max">
                <Text className="title" type="secondary" weight={600}>
                  Supplied balance
                </Text>
                <Text className="main-data" weight={600}>
                  ${supplied}
                </Text>
              </Column>
              <Column crossAxisSize="max">
                <Text className="title" type="secondary" weight={600}>
                  {vm.mode === 0 ? "Supply APR" : "Borrow APR"}
                </Text>
                <Text className="data" weight={600}>
                  {vm.mode === 0 ? vm.supplyApr : vm.supplyApr}
                </Text>
              </Column>
            </Row>
            <Column crossAxisSize="max">
              <Text
                className="title"
                textAlign="end"
                type="secondary"
                weight={600}
              >
                Borrow balance
              </Text>
              <Text textAlign="end" className="data" weight={600}>
                ${borrowed}
              </Text>
            </Column>
          </>
        )
      ) : !vm.initialized ? (
        <Column crossAxisSize="max">
          <Column>
            <Skeleton height={24} width={100} />
            <Skeleton height={48} width={200} />
          </Column>
          <SizedBox height={16} />
          <Divider />
          <SizedBox height={16} />
          <Row justifyContent="space-between" alignItems="center">
            <Column>
              <Skeleton height={24} width={100} />
              <Skeleton height={32} width={100} />
            </Column>
            <Column>
              <Skeleton height={24} width={100} />
              <Skeleton height={32} width={100} />
            </Column>
          </Row>
        </Column>
      ) : (
        <>
          <Column>
            <Text className="title" type="secondary" weight={600}>
              Supplied balance
            </Text>
            <Text className="main-data" weight={600}>
              ${supplied}
            </Text>
          </Column>
          <SizedBox height={16} />
          <Divider />
          <SizedBox height={16} />
          <Row alignItems="center" justifyContent="space-between">
            <Column>
              <Text className="title" type="secondary" weight={600}>
                {vm.mode === 0 ? "Supply APR" : "Borrow APR"}
              </Text>
              <Text className="data" weight={600}>
                {vm.mode === 0 ? vm.supplyApr : vm.supplyApr}
              </Text>
            </Column>
            <Column style={{ textAlign: "end" }}>
              <Text className="title" type="secondary" weight={600}>
                Borrow balance
              </Text>
              <Text className="data" weight={600}>
                ${borrowed}
              </Text>
            </Column>
          </Row>
        </>
      )}
    </Root>
  );
};
export default observer(DashboardStats);
