import React from "react";
import styled from "@emotion/styled";
import DarkMode from "@components/Header/DarkMode";
import Divider from "./Divider";
import { Anchor } from "@components/Anchor";
import SizedBox from "@components/SizedBox";

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

  a {
    font-size: 13px;
    line-height: 16px;
    font-weight: 600;
    color: ${({ theme }) => theme.colors?.neutral4};
  }
`;

const Footer: React.FC<IProps> = () => {
  return (
    <Root>
      <Divider />
      <Row>
        <div style={{ display: "flex", flexDirection: "row" }}>
          <Anchor href="https://docs.swaylend.com" type="secondary">
            Docs
          </Anchor>
          <SizedBox width={12} />
          <Anchor href="https://discord.gg/eC97a9U2Pe" type="secondary">
            Need help?
          </Anchor>
        </div>
        <DarkMode />
      </Row>
    </Root>
  );
};
export default Footer;
