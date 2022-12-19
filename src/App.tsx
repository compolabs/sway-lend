import React from "react";
import styled from "@emotion/styled";
import { observer } from "mobx-react-lite";
import { Column } from "@components/Flex";
import Header from "@components/Header/Header";
import WalletModal from "./components/Wallet/WalletModal";
import { useStores } from "@stores";
import { Routes, Route } from "react-router-dom";
import { ROUTES } from "@src/constants";
import Faucet from "@screens/Faucet";

const Root = styled(Column)`
  width: 100%;
  align-items: center;
  background: ${({ theme }) => theme.colors.primary50};
  min-height: 100vh;
`;
const App: React.FC = () => {
  const { settingsStore } = useStores();
  return (
    <Root>
      <Header />
      <Routes>
        {/* Landing */}
        <Route path={ROUTES.FAUCET} element={<Faucet />} />
      </Routes>
      <WalletModal
        onClose={() => settingsStore.setWalletModalOpened(false)}
        visible={settingsStore.walletModalOpened}
      />
    </Root>
  );
};

export default observer(App);
