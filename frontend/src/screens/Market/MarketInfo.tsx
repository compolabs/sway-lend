import styled from "@emotion/styled";
import React from "react";
import Card from "@components/Card";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import useWindowSize from "@src/hooks/useWindowSize";

interface IProps {}

const Root = styled.div`
  display: grid;
  width: 100%;
  gap: 8px;
  @media (min-width: 880px) {
    grid-template-rows: 1fr;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 16px;
  }
`;
const MarketInfo: React.FC<IProps> = () => {
  const data = [
    { title: "Total liquidity", value: "$ 354,285.32" },
    { title: "Supply APY", value: "60.17%" },
    { title: "Borrow APY", value: "12.32%" },
  ];
  const { width } = useWindowSize();
  return (
    <Root>
      {data.map(({ title, value }) => (
        <Card key={title}>
          <Text
            size={width && width >= 880 ? "semi-big" : undefined}
            type="secondary"
          >
            {title}
          </Text>
          <SizedBox height={4} />
          <Text size="big" weight={500}>
            {value}
          </Text>
        </Card>
      ))}
    </Root>
  );
};
export default MarketInfo;
