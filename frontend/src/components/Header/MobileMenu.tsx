import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import Divider from "../Divider";
import DarkMode from "./DarkMode";
import LinkGroup from "../LinkGroup";
import { Column } from "../Flex";
import Scrollbar from "../Scrollbar";
import Wallet from "../Wallet";
import { ROUTES } from "@src/constants";

interface IProps {
  onClose: () => void;
  opened: boolean;
}

const Root = styled.div<{ opened: boolean }>`
  z-index: 100;
  background: rgba(0, 0, 0, 0.4);
  position: absolute;
  top: 64px;
  left: 0;
  right: 0;
  height: calc(100vh - 64px);
  transition: 0.2s;
  overflow: hidden;

  ${({ opened }) => (!opened ? `height: 0px;` : "")}
  .menu-body {
    display: flex;
    flex-direction: column;
    background: ${({ theme }) => theme.colors.white};
  }
`;

const WalletWrapper = styled.div`
  padding: 24px;
  border-top: 1px solid ${({ theme }) => theme.colors.primary100};
`;

const MobileMenu: React.FC<IProps> = ({ opened, onClose }) => {
  const mainFunctional = [
    { name: "Faucet", link: ROUTES.FAUCET, outer: false },
    { name: "Twitter", link: "https://twitter.com/swaygangsters", outer: true },
    { name: "Discord", link: "https://discord.gg/VHgEGXjF", outer: true },
  ];
  return (
    <Root {...{ opened }}>
      <div className="menu-body">
        <Divider />
        <Scrollbar style={{ margin: 24, marginBottom: 0 }}>
          <Column crossAxisSize="max" style={{ maxHeight: "50vh" }}>
            <LinkGroup onClick={onClose} title="" links={mainFunctional} />
          </Column>
        </Scrollbar>
        <DarkMode style={{ margin: 16 }} />
        <WalletWrapper>
          <Wallet />
        </WalletWrapper>
      </div>
    </Root>
  );
};
export default observer(MobileMenu);
