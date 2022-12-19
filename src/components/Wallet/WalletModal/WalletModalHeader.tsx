import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import { Column } from "@components/Flex";
import Text from "@components/Text";
import { ReactComponent as Copy } from "@src/assets/icons/copy.svg";
import { ReactComponent as Link } from "@src/assets/icons/whiteLink.svg";
import { ReactComponent as Disconnect } from "@src/assets/icons/disconnect.svg";
import { observer } from "mobx-react-lite";
import { useWalletVM } from "@components/Wallet/WalletModal/WalletVM";
import { themes } from "@src/themes/ThemeProvider";

interface IProps {}

const Root = styled(Column)<{ headerExpanded: boolean }>`
  align-items: center;
  justify-content: center;
  width: 100%;
  transition: 0.4s;
  overflow: hidden;
  padding: 0 24px;
  box-sizing: border-box;
  height: ${({ headerExpanded }) => (headerExpanded ? "212px" : "0px")};
  @media (min-width: 560px) {
    height: ${({ headerExpanded }) => (headerExpanded ? "212px" : "0px")};
  }
`;
const Actions = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  column-gap: 8px;
  width: 100%;
`;
const Action = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background: rgba(255, 255, 255, 0.12);
  border-radius: 8px;
  cursor: pointer;
  height: 60px;
  box-sizing: border-box;

  .img {
    height: 24px;
    min-width: 24px;
  }
`;

const WalletModalHeader: React.FC<IProps> = () => {
  const vm = useWalletVM();
  const action = [
    {
      icon: <Copy className="img" />,
      text: "Copy address",
      onClick: vm.handleCopyAddress,
    },
    {
      icon: <Link className="img" />,
      text: "View on Explorer",
    },
    {
      icon: <Disconnect className="img" />,
      text: "Disconnect",
      onClick: vm.handleLogOut,
    },
  ];
  const whiteText = { color: themes.lightTheme.colors.white };
  return (
    <Root headerExpanded={vm.headerExpanded}>
      <Column alignItems="center" crossAxisSize="max">
        <Text fitContent size="medium" style={whiteText}>
          {vm.signInInfo}
        </Text>
        <Text fitContent size="large" style={whiteText}>
          $&nbsp;100
        </Text>
        <SizedBox height={16} />
        <Actions>
          {action.map(({ text, icon, onClick }) => (
            <Action onClick={onClick} key={text}>
              {icon}
              <SizedBox height={6} />
              <Text size="small" style={whiteText} fitContent>
                {text}
              </Text>
            </Action>
          ))}
        </Actions>
      </Column>
    </Root>
  );
};
export default observer(WalletModalHeader);
