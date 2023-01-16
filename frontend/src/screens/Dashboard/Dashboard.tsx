import styled from "@emotion/styled";
import React from "react";
import { Observer } from "mobx-react-lite";
import { DashboardVMProvider } from "@screens/Dashboard/DashboardVm";
import Layout from "@components/Layout";
import DashboardStats from "@screens/Dashboard/DashboardStats";
import SizedBox from "@components/SizedBox";
import AssetsTable from "@screens/Dashboard/AssetsTable";
import ActionTab from "@screens/Dashboard/ActionTab";

interface IProps {}

const Root = styled.div<{ apySort?: boolean; liquiditySort?: boolean }>`
  display: flex;
  flex-direction: column;
  justify-content: center;
  box-sizing: border-box;
  padding: 0 16px;
  width: 100%;
  min-height: 100%;
  margin-bottom: 24px;
  margin-top: 40px;
  text-align: left;
  @media (min-width: 880px) {
    margin-top: 56px;
  }

  .apy-group {
    width: 20px;
    height: 20px;
    transform: ${({ apySort }) => (apySort ? "scale(1)" : "scale(1, -1)")};
  }

  .liquidity-group {
    width: 20px;
    height: 20px;
    transform: ${({ liquiditySort }) =>
      liquiditySort ? "scale(1)" : "scale(1, -1)"};
  }
`;
const MainContainer = styled.div`
  width: 100%;
  display: grid;
  @media (min-width: 880px) {
    grid-template-columns: 3fr 1fr;
    column-gap: 20px;
  }
`;
const DashboardImpl: React.FC<IProps> = () => {
  return (
    <Layout>
      <Observer>
        {() => (
          <Root>
            <DashboardStats />
            <SizedBox height={24} />
            <MainContainer>
              <AssetsTable />
              <ActionTab />
            </MainContainer>
          </Root>
        )}
      </Observer>
    </Layout>
  );
};

const Dashboard: React.FC<IProps> = () => (
  <DashboardVMProvider>
    <DashboardImpl />
  </DashboardVMProvider>
);
export default Dashboard;
