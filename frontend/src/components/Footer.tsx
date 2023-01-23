import React from "react";
import styled from "@emotion/styled";
import Text from "@components/Text";
import DarkMode from "@components/Header/DarkMode";
import Divider from "./Divider";

interface IProps {}

const Root = styled.footer`
  display: flex;
  flex-direction: column;
  align-items: center;
  box-sizing: border-box;
  padding: 0 16px;
  //todo add mobile
  max-width: 1300px;
  @media (min-width: 880px) {
  }

  width: 100%;
`;
const Row = styled.div`
  display: flex;
  width: 100%;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  @media (min-width: 880px) {
    padding: 20px 0;
  }
`;

const Footer: React.FC<IProps> = () => {
  return (
    <Root>
      <Divider />
      <Row>
        <Text fitContent weight={600} size="small" type="secondary">
          Terms
        </Text>
        <DarkMode />
      </Row>
    </Root>
  );
};
export default Footer;
