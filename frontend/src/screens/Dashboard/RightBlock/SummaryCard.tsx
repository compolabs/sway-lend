import React from "react";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Divider from "@components/Divider";
import { Row } from "@components/Flex";
import Card from "@components/Card";

interface IProps {}

const SummaryCard: React.FC<IProps> = () => {
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
      {stats.map(({ value, title }) => (
        <Row style={{ marginBottom: 12 }}>
          <Text weight={600}>{title}</Text>
          <Text textAlign="right" weight={600}>
            {value}
          </Text>
        </Row>
      ))}
    </Card>
  );
};
export default SummaryCard;
