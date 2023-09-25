import React from "react";
import Layout from "@components/Layout";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import TutorialCard from "@screens/Tutorial/TutorialCard";
import useWindowSize from "@src/hooks/useWindowSize";

interface IProps {}

// const Root = styled.div`
//   display: flex;
//   flex-direction: column;
//   max-width: 310px;
// `;
const cards = [
  {
    title: "#0: Wallet connection",
    complexity: "Low",
    time: "10 min",
    pic: "string",
  },
  {
    title: "#1: Mint of ETH and collateral",
    complexity: "Medium",
    time: "15 min",
    pic: "string",
  },
  {
    title: "#2: Supply of the collateral",
    complexity: "Low",
    time: "10 min",
    pic: "string",
  },
  {
    title: "#3: Borrow",
    complexity: "Low",
    time: "10 min",
    pic: "string",
  },
];
const Container = styled.div`
  ////display: grid;
  ////gap: 16px;
  ////grid-template-rows: repeat(auto-fit, minmax(310px, 4fr));
  //display: flex;
  //width: 100%;
  //flex-direction: row;
  //flex-wrap: wrap;
  ////justify-content: space-around;
  //
  //& > * {
  //  margin-right: 16px;
  //}
`;
const Tutorial: React.FC<IProps> = () => {
  const { width } = useWindowSize();
  return (
    <Layout>
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
    </Layout>
  );
};

export default Tutorial;
