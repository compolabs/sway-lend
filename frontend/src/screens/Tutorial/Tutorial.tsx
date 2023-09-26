import React from "react";
import Layout from "@components/Layout";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import TutorialCard from "@screens/Tutorial/TutorialCard";
import useWindowSize from "@src/hooks/useWindowSize";
import walletConnect from "@src/assets/tutorials/walletConnect.png";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  padding: 16px;
`;
const cards = [
  {
    title: "#0: Wallet connection",
    complexity: "Low",
    time: "10 min",
    pic: walletConnect,
  },
  {
    title: "#1: Mint of ETH and collateral",
    complexity: "Medium",
    time: "15 min",
    pic: walletConnect,
  },
  {
    title: "#2: Supply of the collateral",
    complexity: "Low",
    time: "10 min",
    pic: walletConnect,
  },
  {
    title: "#3: Borrow",
    complexity: "Low",
    time: "10 min",
    pic: walletConnect,
  },
];
const Container = styled.div`
  display: inline-grid;
  gap: 16px;
  grid-template-columns: 1fr;
  @media (min-width: 768px) {
    grid-template-columns: 1fr 1fr;
  }
  @media (min-width: 1024px) {
    grid-template-columns: 1fr 1fr 1fr 1fr;
  }
`;
const Tutorial: React.FC<IProps> = () => {
  const { width } = useWindowSize();
  return (
    <Layout>
      <Root>
        <SizedBox height={40} />
        <Text weight={600} size={width && width >= 880 ? "large" : "big"}>
          Participate on️ the Swaylend challenge
        </Text>
        <SizedBox height={32} />
        <Container>
          {cards.map((card) => (
            <TutorialCard {...card} />
          ))}
        </Container>
      </Root>
    </Layout>
  );
};

export default Tutorial;
