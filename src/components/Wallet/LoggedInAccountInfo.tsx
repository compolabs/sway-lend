import styled from "@emotion/styled";
import React, { useState } from "react";
import { Row } from "@components/Flex";
import { ReactComponent as WalletIcon } from "@src/assets/icons/wallet.svg";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import arrowIcon from "@src/assets/icons/arrow.svg";
import * as identityImg from "identity-img";
import { useStores } from "@stores";
import Tooltip from "@components/Tooltip";
import { observer } from "mobx-react-lite";
import WalletActionsTooltip from "./WalletActionsTooltip";
import centerEllipsis from "@src/utils/centerEllipsis";

interface IProps {}

const Root = styled(Row)`
  align-items: center;
  height: fit-content;
  justify-content: space-between;
  @media (min-width: 880px) {
    justify-content: flex-end;
  }

  .balances {
    display: flex;
    align-items: center;
    cursor: pointer;
  }
`;

const AddressContainer = styled.div<{ expanded: boolean }>`
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  padding: 0 16px;
  box-sizing: border-box;
  border: 1px solid ${({ theme }) => theme.colors.primary100};
  border-radius: 10px;
  cursor: pointer;
  background: ${({ expanded, theme }) =>
    expanded ? theme.colors.primary100 : theme.colors.white};

  :hover {
    background: ${({ theme }) => theme.colors.primary100};
  }

  .avatar {
    transition: 0.4s;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    margin-right: 8px;
  }

  .menu-arrow {
    transition: 0.4s;
    transform: ${({ expanded }) =>
      expanded ? "rotate(-90deg)" : "rotate(90deg)"};
  }
`;

const LoggedInAccountInfo: React.FC<IProps> = () => {
  const { accountStore, settingsStore } = useStores();
  const { address } = accountStore;
  const avatar = address && identityImg.create(address, { size: 24 * 3 });
  const [accountOpened, setAccountOpened] = useState<boolean>(false);
  return (
    <Root>
      <WalletIcon
        onClick={() => settingsStore.setWalletModalOpened(true)}
        style={{ cursor: "pointer" }}
      />
      <SizedBox width={24} />
      <Tooltip
        config={{
          placement: "bottom-end",
          trigger: "click",
          onVisibleChange: setAccountOpened,
        }}
        content={<WalletActionsTooltip address={address!} />}
      >
        <AddressContainer expanded={accountOpened}>
          <img className="avatar" src={avatar!} alt="avatar" />
          <Text>{centerEllipsis(address ?? "", 10)}</Text>
          <SizedBox width={10} />
          <img src={arrowIcon} className="menu-arrow" alt="arrow" />
        </AddressContainer>
      </Tooltip>
    </Root>
  );
};
export default observer(LoggedInAccountInfo);
