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
  if (vm.mode === 0) return null;
  const stats = [
    {
      title: "Supply APR",
      value: vm.supplyApr,
    },
    {
      title: "Borrow APR",
      value: vm.borrowApr,
    },
    { title: "Total liquidity", value: vm.totalLiquidity },
  ];
  return (
    <Card>
      <Text weight={600} type="secondary" size="small">
        Position summary
      </Text>
      <SizedBox height={16} />
      <Divider />
      <SizedBox height={12} />
      {stats.map(({ value, title }, index) => (
        <Row key={index} style={{ marginBottom: 12 }}>
          <Text weight={600}>{title}</Text>
          {value == null ? (
            <Skeleton height={24} width={100} />
          ) : (
            <Text textAlign="right" weight={600}>
              {value}
            </Text>
          )}
        </Row>
      ))}
    </Card>
  );
};
export default observer(SummaryCard);
