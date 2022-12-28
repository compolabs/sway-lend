import styled from "@emotion/styled";
import React, { useState } from "react";
import mobileMenuIcon from "@src/assets/icons/mobileMenu.svg";
import closeIcon from "@src/assets/icons/close.svg";
import { Column, Row } from "@components/Flex";
import MobileMenu from "@components/Header/MobileMenu";
import SizedBox from "@components/SizedBox";
import { observer } from "mobx-react-lite";
import { useLocation, useNavigate } from "react-router-dom";
import { Anchor } from "@components/Anchor";
import Tooltip from "@components/Tooltip";
import DarkMode from "@components/Header/DarkMode";
import isRoutesEquals from "@src/utils/isRoutesEquals";
import LinkGroup from "../LinkGroup";
import Wallet from "../Wallet";
import { ROUTES } from "@src/constants";
import { useTheme } from "@emotion/react";

interface IProps {}

const Root = styled(Column)`
  width: 100%;
  background: ${({ theme }) => theme.colors.white};
  align-items: center;
  z-index: 102;
  box-shadow: 0 8px 56px rgba(54, 56, 112, 0.16);

  //todo check
  a {
    text-decoration: none;
  }
`;

const TopMenu = styled.header`
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 64px;
  padding: 0 16px;
  max-width: 1440px;
  z-index: 102;
  @media (min-width: 880px) {
    height: 80px;
  }
  box-sizing: border-box;
  background: ${({ theme }) => theme.colors.white};

  .logo {
    height: 30px;
    @media (min-width: 880px) {
      height: 36px;
    }
  }

  .icon {
    cursor: pointer;
  }
`;

const MenuItem = styled(Anchor)<{ selected?: boolean }>`
  cursor: pointer;
  display: flex;
  align-items: center;
  font-weight: 500;
  font-size: 16px;
  line-height: 24px;
  color: ${({ selected, theme }) =>
    selected ? theme.colors.primary800 : theme.colors.primary650};
  box-sizing: border-box;
  border-bottom: 4px solid
    ${({ selected, theme }) =>
      selected ? theme.colors.blue500 : "transparent"};
  height: 100%;
  margin: 0 12px;
  transition: 0.4s;

  &:hover {
    border-bottom: 4px solid ${({ theme }) => theme.colors.primary300};
    color: ${({ theme }) => theme.colors.blue500};
  }
`;

const Mobile = styled.div`
  display: flex;
  min-width: fit-content;
  @media (min-width: 880px) {
    display: none;
  }
`;

const Desktop = styled.div`
  display: none;
  min-width: fit-content;
  @media (min-width: 880px) {
    height: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }
`;

const Header: React.FC<IProps> = () => {
  const [mobileMenuOpened, setMobileMenuOpened] = useState(false);
  const location = useLocation();
  const toggleMenu = (state: boolean) => {
    window.scrollTo({ top: 0, behavior: "smooth" });
    document.body.classList.toggle("noscroll", state);
    setMobileMenuOpened(state);
  };
  const navigate = useNavigate();

  const menuItems = [
    { name: "Market", link: ROUTES.ROOT },
    { name: "Faucet", link: ROUTES.FAUCET },
  ];

  const communityMenu = [
    { name: "Discord", link: "https://discord.gg/VHgEGXjF", outer: true },
    { name: "Medium", link: "https://discord.gg/VHgEGXjF", outer: true },
    { name: "Twitter", link: "https://twitter.com/swaygangsters", outer: true },
  ];
  const theme = useTheme();
  return (
    <Root>
      <Mobile>
        <MobileMenu
          opened={mobileMenuOpened}
          onClose={() => toggleMenu(false)}
        />
      </Mobile>

      <TopMenu>
        <Row alignItems="center" crossAxisSize="max">
          <a href="https://app.swaylend.com">
            <img className="logo" src={theme.images.icons.logo} alt="logo" />
          </a>
          <Desktop>
            <SizedBox width={54} />
            {menuItems.map(({ name, link }) => (
              <MenuItem
                key={name}
                selected={isRoutesEquals(link, location.pathname)}
                onClick={() => navigate(link)}
              >
                {name}
              </MenuItem>
            ))}
          </Desktop>
        </Row>
        <Mobile>
          <img
            onClick={() => toggleMenu(!mobileMenuOpened)}
            className="icon"
            src={mobileMenuOpened ? closeIcon : mobileMenuIcon}
            alt="menuControl"
          />
        </Mobile>
        <Desktop>
          <Wallet />
          <SizedBox width={24} />
          <Tooltip
            config={{
              placement: "bottom-start",
              trigger: "click",
            }}
            content={
              <Column crossAxisSize="max">
                <LinkGroup title="" links={communityMenu} />
                <SizedBox height={8} />
                <DarkMode />
              </Column>
            }
          >
            <img
              onClick={() => toggleMenu(!mobileMenuOpened)}
              className="icon"
              src={mobileMenuIcon}
              alt="menuControl"
            />
          </Tooltip>
        </Desktop>
      </TopMenu>
    </Root>
  );
};
export default observer(Header);
