import styled from "@emotion/styled";
import React, { useState } from "react";
import { Column, Row } from "@components/Flex";
import { observer } from "mobx-react-lite";
import Scrollbar from "@components/Scrollbar";
import { useWalletVM } from "@components/Wallet/WalletModal/WalletVM";
import SizedBox from "@components/SizedBox";
import Tabs from "@components/Tabs";
import AssetsBalances from "./AssetBalances";

interface IProps {}

const Root = styled(Column)`
  width: 100%;
  box-sizing: border-box;
  background: ${({ theme }) => theme.colors.white};

  & > * {
    width: 100%;
  }
`;

const TabsWrapper = styled(Row)`
  border-radius: 16px 16px 0px 0px;
  background: ${({ theme }) => theme.colors.white};
  height: 56px;
  margin-top: -56px;
`;

const ListWrapper = styled.div<{ headerExpanded: boolean }>`
  width: 100%;
  display: flex;
  flex-direction: column;
  transition: 0.4s;
  height: ${({ headerExpanded }) =>
    headerExpanded ? "calc(100vh - 212px)" : "calc(100vh - 96px)"};

  @media (min-width: 560px) {
    height: ${({ headerExpanded }) =>
      headerExpanded ? "calc(560px - 212px)" : "calc(560px - 96px)"};
  }
`;

const WalletModalBody: React.FC<IProps> = () => {
  const vm = useWalletVM();
  const handleScroll = (container: HTMLElement) => {
    vm.setHeaderExpanded(container.scrollTop === 0);
  };
  const [activeTab, setActiveTab] = useState<number>(0);
  return (
    <Root>
      <TabsWrapper>
        <Tabs
          tabs={[{ name: "Assets" }, { name: "Investments" }, { name: "NFTs" }]}
          activeTab={activeTab}
          setActive={(v) => setActiveTab(v)}
          style={{ justifyContent: "space-evenly", paddingTop: 16 }}
          tabStyle={{ flex: 1, marginRight: 0 }}
        />
      </TabsWrapper>
      <Scrollbar onScrollY={handleScroll}>
        <ListWrapper headerExpanded={vm.headerExpanded}>
          <SizedBox height={8} />
          {activeTab === 0 && <AssetsBalances />}
          <SizedBox height={64} width={1} />
        </ListWrapper>
      </Scrollbar>
    </Root>
  );
};
export default observer(WalletModalBody);
