import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import Divider from "../Divider";
import DarkMode from "./DarkMode";
import { Column } from "../Flex";
import Wallet from "../Wallet";
import { ROUTES } from "@src/constants";
import { useTheme } from "@emotion/react";
import isRoutesEquals from "@src/utils/isRoutesEquals";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import { useLocation, useNavigate } from "react-router-dom";

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

  ${({ opened }) => !opened && `height: 0px;`}
`;
const Body = styled.div`
  display: flex;
  flex-direction: column;
  background: ${({ theme }) => theme.colors.mainBackground};
`;

const WalletWrapper = styled.div`
  padding: 16px;
`;

const MenuItem = styled.div<{ selected?: boolean }>`
  display: flex;
  cursor: pointer;
  flex-direction: row;
  //justify-content: center;
  //align-items: center;
  padding: 12px 16px;
  border-radius: 4px;
  width: 100%;
  background: ${({ selected, theme }) =>
    selected && theme.colors.header.navLinkBackground};

  &:hover {
  }
`;

const Container = styled(Column)`
  //padding: 16px;
  //background: pink;
  margin: 16px;

  & > * {
    margin-bottom: 8px;
  }
`;
const MobileMenu: React.FC<IProps> = ({ opened, onClose }) => {
  const theme = useTheme();
  const navigate = useNavigate();
  const location = useLocation();

  const menuItems = [
    {
      name: "Dashboard",
      link: ROUTES.DASHBOARD,
      icon: theme.images.icons.dashboard,
    },
    {
      name: "Faucet",
      link: ROUTES.FAUCET,
      icon: theme.images.icons.analytics,
    },
  ];
  return (
    <Root {...{ opened }}>
      <Body>
        <Container crossAxisSize="max" style={{ maxHeight: "50vh" }}>
          {menuItems.map(({ name, link, icon }) => (
            <MenuItem
              key={name}
              selected={isRoutesEquals(link, location.pathname)}
              onClick={() => navigate(link)}
            >
              <img style={{ width: 24, height: 24 }} src={icon} alt="nav" />
              <SizedBox width={4} />
              <Text weight={700}>{name}</Text>
            </MenuItem>
          ))}
        </Container>
        <Divider />
        <DarkMode text style={{ margin: 16 }} />
        <WalletWrapper>
          <Wallet />
        </WalletWrapper>
      </Body>
    </Root>
  );
};
export default observer(MobileMenu);
