import React from "react";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Divider from "@components/Divider";
import { Row } from "@components/Flex";
import Card from "@components/Card";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import Skeleton from "react-loading-skeleton";

interface IProps {}

const SummaryCard: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const stats = [
    {
      title: "Supply APR",
      value: vm.supplyApr,
      changeValue: vm.possibleSupplyApr,
    },
    {
      title: "Borrow APR",
      value: vm.borrowApr,
      changeValue: vm.possibleBorrowApr,
    },
    {
      title: "Total liquidity",
      value: vm.totalLiquidity,
      changeValue: null,
    },
  ];

  return (
    <Card>
      <Text weight={600} type="secondary" size="small">
        Position summary
      </Text>
      <SizedBox height={16} />
      <Divider />
      <SizedBox height={12} />
      {stats.map(({ value, title, changeValue }, index) => (
        <Row key={index} style={{ marginBottom: 12 }}>
          <Text weight={600}>{title}</Text>
          {value == null ? (
            <Skeleton height={24} width={100} />
          ) : (
            <>
              {changeValue != null ? (
                <Text
                  textAlign="right"
                  style={{ color: "#00b493" }}
                  weight={600}
                >
                  {changeValue}
                </Text>
              ) : (
                <Text textAlign="right" weight={600}>
                  {value}
                </Text>
              )}
            </>
          )}
        </Row>
      ))}
    </Card>
  );
};
export default observer(SummaryCard);
