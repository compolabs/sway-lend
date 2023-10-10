import React from "react";
import Layout from "@components/Layout";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import useWindowSize from "@src/hooks/useWindowSize";
import tutorials from "@src/constants/tutorials";
import TutorialCard from "./TutorialCard";
import { useNavigate } from "react-router-dom";
import { ROUTES } from "@src/constants";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  padding: 16px;
`;
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
const Tutorials: React.FC<IProps> = () => {
  const { width } = useWindowSize();
  const navigate = useNavigate();
  return (
    <Layout>
      <Root>
        <SizedBox height={40} />
        <Text weight={600} size={width && width >= 880 ? "large" : "big"}>
          Participate in the Swaylend Challenge
        </Text>
        <SizedBox height={32} />
        <Container>
          {tutorials.map((card) => (
            <TutorialCard
              {...card}
              onClick={() =>
                navigate({ pathname: `${ROUTES.TUTORIALS}/${card.id}` })
              }
            />
          ))}
        </Container>
      </Root>
    </Layout>
  );
};

export default Tutorials;
