import styled from "@emotion/styled";
import React from "react";
import { Column } from "@components/Flex";
import Text from "@components/Text";
import copy from "copy-to-clipboard";
import { observer } from "mobx-react-lite";
import { useStores } from "@stores";
import { Anchor } from "@components/Anchor";
import { EXPLORER_URL } from "@src/constants";
import SizedBox from "@components/SizedBox";

interface IProps {
  address: string;
}

const Root = styled(Column)`
  padding: 16px;

  .menu-item {
    padding: 12px 20px;
    cursor: pointer;

    :hover {
      background: ${({ theme }) => theme.colors.tooltip.hoverElement};
      border-radius: 4px;
    }
  }
`;

const WalletActionsTooltip: React.FC<IProps> = ({ address }) => {
  const { notificationStore, accountStore } = useStores();

  const handleCopyAddress = () => {
    address && copy(address);
    notificationStore.notify("Your address was copied", {
      type: "success",
      title: "Congratulations!",
    });
  };
  const handleLogout = () => accountStore.disconnect();

  return (
    <Root alignItems="center">
      <Text weight={700} onClick={handleCopyAddress} className="menu-item">
        Copy address
      </Text>
      <SizedBox height={10} />
      <Text
        className="menu-item"
        onClick={() => window.open(`${EXPLORER_URL}/address/${address}`)}
        weight={700}
      >
        View in Explorer
      </Text>
      <SizedBox height={10} />
      <Text weight={700} onClick={handleLogout} className="menu-item">
        Disconnect
      </Text>
    </Root>
  );
};
export default observer(WalletActionsTooltip);
