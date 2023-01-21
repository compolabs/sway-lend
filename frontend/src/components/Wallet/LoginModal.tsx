import React from "react";
import Dialog from "@components/Dialog";
import { LOGIN_TYPE } from "@stores/AccountStore";
import LoginType from "./LoginType";
import fuel from "@src/assets/icons/fuelLogo.svg";
import Text from "@components/Text";
import { observer } from "mobx-react-lite";
import Img from "@components/Img";
import sway from "@src/assets/tokens/sway.svg";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";

interface IProps {
  onClose: () => void;
  onLogin: (loginType: LOGIN_TYPE) => void;
  visible: boolean;
}

const loginTypes = [
  {
    title: "Generate wallet",
    icon: fuel,
    type: LOGIN_TYPE.GENERATE_FROM_SEED,
  },
  // {
  //   title: "Fuel",
  //   icon: fuel,
  //   type: LOGIN_TYPE.FUEL_WALLET,
  // },
];
const Root = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`;
const LoginModal: React.FC<IProps> = ({ onLogin, ...rest }) => {
  const handleLogin = (loginType: LOGIN_TYPE) => () => {
    onLogin(loginType);
    rest.onClose();
  };
  // const isThereWallet = window?.fuel != null;
  // const walletLink = "https://fuels-wallet.vercel.app/docs/install/";
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
          To start using Swaylend
        </Text>
        <SizedBox height={34} />
        {loginTypes.map((t) => (
          <LoginType {...t} key={t.type} onClick={handleLogin(t.type)} />
        ))}

        {/*{isThereWallet ? (*/}
        {/*  loginTypes.map((t) => (*/}
        {/*    <LoginType {...t} key={t.type} onClick={handleLogin(t.type)} />*/}
        {/*  ))*/}
        {/*) : (*/}
        {/*  <Notification*/}
        {/*    type="warning"*/}
        {/*    text={*/}
        {/*      <>*/}
        {/*        <Text>Fuel Wallet hasn't been detected.</Text>*/}
        {/*        <Anchor href={walletLink}>You can download it here</Anchor>*/}
        {/*      </>*/}
        {/*    }*/}
        {/*  />*/}
        {/*)}*/}
        {/*<SizedBox height={40} />*/}
      </Root>
    </Dialog>
  );
};
export default observer(LoginModal);
