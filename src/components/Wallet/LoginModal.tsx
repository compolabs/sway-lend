import React from "react";
import Dialog from "@components/Dialog";
import { LOGIN_TYPE } from "@stores/AccountStore";
import LoginType from "./LoginType";
import fuel from "@src/assets/icons/fuelLogo.svg";
import { observer } from "mobx-react-lite";

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
  return (
    <Dialog style={{ maxWidth: 360 }} title="Connect wallet" {...rest}>
      {loginTypes.map((t) => (
        <LoginType {...t} key={t.type} onClick={handleLogin(t.type)} />
      ))}
    </Dialog>
  );
};
export default observer(LoginModal);
