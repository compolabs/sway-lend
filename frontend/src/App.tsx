import React from "react";
import styled from "@emotion/styled";
import { observer } from "mobx-react-lite";
import { Column } from "@components/Flex";
import Header from "@components/Header/Header";
import WalletModal from "./components/Wallet/WalletModal";
import { useStores } from "@stores";
import { Navigate, Route, Routes } from "react-router-dom";
import { ROUTES } from "@src/constants";
import Faucet from "@screens/Faucet";
import Dashboard from "@screens/Dashboard";
import Footer from "@components/Footer";

const Root = styled(Column)`
  width: 100%;
  align-items: center;
  background: ${({ theme }) => theme.colors.mainBackground};
  min-height: 100vh;
  min-width: 1440px;
`;
const App: React.FC = () => {
  const { settingsStore } = useStores();
  return (
    <Root>
      <Header />
      <Routes>
        <Route path={ROUTES.DASHBOARD} element={<Dashboard />} />
        <Route path={ROUTES.FAUCET} element={<Faucet />} />
        <Route path="*" element={<Navigate to={ROUTES.DASHBOARD} />} />
      </Routes>
      <Footer />
      <WalletModal
        onClose={() => settingsStore.setWalletModalOpened(false)}
        visible={settingsStore.walletModalOpened}
      />
    </Root>
  );
};

export default observer(App);
