import React from "react";
import Dialog from "@components/Dialog";
import { LOGIN_TYPE } from "@stores/AccountStore";
import LoginType from "./LoginType";
import fuel from "@src/assets/icons/fuelLogo.svg";
import Notification from "@components/Notification";
import Text from "@components/Text";
import { observer } from "mobx-react-lite";
import { Anchor } from "@components/Anchor";

interface IProps {
  onClose: () => void;
  onLogin: (loginType: LOGIN_TYPE) => void;
  visible: boolean;
}

const loginTypes = [
  {
    title: "Fuel wallet",
    icon: fuel,
    type: LOGIN_TYPE.FUEL_WALLET,
  },
];
const LoginModal: React.FC<IProps> = ({ onLogin, ...rest }) => {
  const handleLogin = (loginType: LOGIN_TYPE) => () => {
    rest.onClose();
    onLogin(loginType);
  };
  const isThereWallet = window?.fuel == null;
  const walletLink = "https://fuels-wallet.vercel.app/docs/install/";
  return (
    <Dialog style={{ maxWidth: 360 }} title="Connect wallet" {...rest}>
      {!isThereWallet ? (
        loginTypes.map((t) => (
          <LoginType {...t} key={t.type} onClick={handleLogin(t.type)} />
        ))
      ) : (
        <Notification
          type="warning"
          text={
            <>
              <Text>Fuel Wallet hasn't been detected.</Text>
              <Anchor href={walletLink}>You can download it here</Anchor>
            </>
          }
        />
      )}
    </Dialog>
  );
};
export default observer(LoginModal);
