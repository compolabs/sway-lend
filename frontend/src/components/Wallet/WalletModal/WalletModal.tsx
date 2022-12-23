import styled from "@emotion/styled";
import React from "react";
import RcDialog from "rc-dialog";
import "./wallet.css";
import { IDialogPropTypes } from "rc-dialog/lib/IDialogPropTypes";
import { ReactComponent as CloseIcon } from "@src/assets/icons/close.svg";
import SizedBox from "@components/SizedBox";
import WalletModalHeader from "./WalletModalHeader";
import WalletModalBody from "@components/Wallet/WalletModal/WalletModalBody";
import { WalletVMProvider } from "@components/Wallet/WalletModal/WalletVM";
import { observer } from "mobx-react-lite";

interface IProps extends IDialogPropTypes {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
`;

const WalletModal: React.FC<IProps> = ({ ...rest }) => (
  <RcDialog
    wrapClassName="wallet-dialog"
    closeIcon={<CloseIcon style={{ marginTop: 8 }} />}
    animation="zoom"
    maskAnimation="fade"
    destroyOnClose
    {...rest}
  >
    <WalletVMProvider>
      <Root>
        <SizedBox height={48} />
        <WalletModalHeader />
        <SizedBox height={56} />
        <WalletModalBody />
      </Root>
    </WalletVMProvider>
  </RcDialog>
);
export default observer(WalletModal);
