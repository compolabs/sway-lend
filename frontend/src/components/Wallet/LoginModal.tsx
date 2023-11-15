import React from "react";
import Dialog from "@components/Dialog";
import LoginType from "./LoginType";
import Text from "@components/Text";
import { observer } from "mobx-react-lite";
import Img from "@components/Img";
import sway from "@src/assets/tokens/sway.svg";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";
import { useStores } from "@stores";
import { LOGIN_TYPE } from "@stores/AccountStore";

interface IProps {
  onClose: () => void;
  visible: boolean;
}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`;
const LoginModal: React.FC<IProps> = ({ ...rest }) => {
  const { accountStore } = useStores();
  const handleLogin = (type: LOGIN_TYPE | null, active: boolean) => () => {
    if (!active || type == null) return;
    accountStore.login(type);
    rest.onClose();
  };

  const wallets = [
    {
      title: "Fuel Wallet",
      type: LOGIN_TYPE.FUEL_WALLET,
      active: accountStore.listConnectors.includes(LOGIN_TYPE.FUEL_WALLET),
    },
    {
      title: "Fuelet",
      type: LOGIN_TYPE.FUELET,
      active: accountStore.listConnectors.includes(LOGIN_TYPE.FUELET),
    },
    { title: "Create account", type: LOGIN_TYPE.GENERATE_SEED, active: true },
    {
      title: "Fuel Wallet Dev",
      type: LOGIN_TYPE.FUEL_DEV,
      active: accountStore.listConnectors.includes(LOGIN_TYPE.FUEL_DEV),
    },
  ];
  return (
    <Dialog style={{ maxWidth: 360 }} {...rest}>
      <Root>
        <Img height="60" width="60" src={sway} />
        <SizedBox height={4} />
        <Text fitContent weight={600} size="medium">
          Connect wallet
        </Text>
        <SizedBox height={4} />
        <Text fitContent type="secondary" weight={500} size="tiny">
          To start using SwayLend
        </Text>
        <SizedBox height={34} />
        {wallets.map(({ active, title, type }) => (
          <LoginType
            key={title}
            active={active}
            title={title}
            onClick={handleLogin(type, active)}
          />
        ))}
        <SizedBox height={36} />
      </Root>
    </Dialog>
  );
};
export default observer(LoginModal);
