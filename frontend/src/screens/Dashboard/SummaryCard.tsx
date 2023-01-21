import React from "react";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Divider from "@components/Divider";
import { Row } from "@components/Flex";
import Card from "@components/Card";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";

interface IProps {}

const SummaryCard: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  if (vm.mode === 0) return null;
  const stats = [
    { title: "Supply APY", value: "60.17%" },
    { title: "Borrow APY", value: "12.32%" },
    { title: "Total liquidity", value: "$354,285.32" },
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
          <Text textAlign="right" weight={600}>
            {value}
          </Text>
        </Row>
      ))}
    </Card>
  );
};
export default observer(SummaryCard);
