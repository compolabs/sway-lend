import styled from "@emotion/styled";
import React from "react";
import { Column } from "@components/Flex";
import Text from "@components/Text";
import copy from "copy-to-clipboard";
import { observer } from "mobx-react-lite";
import { useStores } from "@stores";
import { Anchor } from "@components/Anchor";
import { EXPLORER_URL } from "@src/constants";

interface IProps {
  address: string;
}

const Root = styled(Column)`
  .menu-item {
    padding: 10px 0;
    cursor: pointer;

    :first-of-type {
      padding-top: 0;
    }

    :last-of-type {
      padding-bottom: 0;
    }
  }

  .divider {
    margin: 0 -16px;
    width: calc(100% + 32px);
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
    <Root>
      <Text weight={500} onClick={handleCopyAddress} className="menu-item">
        Copy address
      </Text>
      <Anchor
        style={{ padding: "10px 0" }}
        href={`${EXPLORER_URL}/address/${address}`}
      >
        <Text weight={500}>View in Explorer</Text>
      </Anchor>
      <Text weight={500} onClick={handleLogout} className="menu-item">
        Disconnect
      </Text>
    </Root>
  );
};
export default observer(WalletActionsTooltip);
