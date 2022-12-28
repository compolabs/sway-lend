import styled from "@emotion/styled";
import Card from "@src/components/Card";
import { Column, Row } from "@src/components/Flex";
import Text from "@src/components/Text";
import React from "react";
import SizedBox from "@components/SizedBox";
import useWindowSize from "@src/hooks/useWindowSize";
import health from "@src/assets/icons/health-gradient.svg";
import healthSmall from "@src/assets/icons/health-gradient-small.svg";
import CircularProgressbar from "@components/CircularProgressbar";
import Tooltip from "@components/Tooltip";
import info from "@src/assets/icons/info.svg";

interface IProps {}

const Root = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;
`;
const Grid = styled.div`
  display: grid;
  width: 100%;
  gap: 8px;
  grid-template-columns: 1fr 1fr;
  @media (min-width: 880px) {
    grid-template-rows: 1fr;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 16px;
  }
  z-index: 2;
`;
//todo add  red color support

const HealthGradient = styled.img`
  position: absolute;
  right: 0;
  top: -1px;
  border-radius: inherit;
`;
const Info = styled.img`
  width: 16px;
  height: 16px;
`;
const AccountHealth: React.FC<IProps> = () => {
  const { width } = useWindowSize();
  const data = [
    {
      title: "Supply balance",
      value: "$ 5,532.32",
      description: "blah blah blah",
    },
    {
      title: "Borrow balance",
      value: "$ 1,621.22",
      description: "blah blah blah",
    },
    { title: "NET APY", value: "32.54%", description: "blah blah blah" },
    {
      title: "Daily reward",
      value: "$ 1,142.21",
      description: "blah blah blah",
    },
  ];
  return (
    <Root>
      <Card style={{ position: "relative" }}>
        <HealthGradient
          src={width && width >= 880 ? health : healthSmall}
          alt="health"
        />
        <Row>
          <CircularProgressbar percent={80} />
          <SizedBox width={16} />
          <Column>
            <Text size="big" weight={500}>
              Account health
            </Text>
            <SizedBox height={4} />
            <Text
              size="small"
              type="secondary"
              weight={500}
              style={{ color: "#7075e9" }}
            >
              Lear more
            </Text>
          </Column>
        </Row>
        <SizedBox height={width && width >= 880 ? 24 : 16} />
        <Grid>
          {data.map(({ title, value, description }) => (
            <Column key={title}>
              <Tooltip
                config={{
                  placement: "bottom-start",
                  trigger: "click",
                }}
                content={<Text>{description}</Text>}
              >
                <Row justifyContent="center" alignItems="center">
                  <Text size="medium" type="secondary">
                    {title}
                  </Text>
                  <SizedBox width={4} />
                  <Info src={info} alt="info" />
                </Row>
              </Tooltip>

              <Text size="big" weight={500}>
                {value}
              </Text>
            </Column>
          ))}
        </Grid>
      </Card>
    </Root>
  );
};
export default AccountHealth;
