import styled from "@emotion/styled";
import React from "react";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import Text from "@components/Text";
import { Column, Row } from "@src/components/Flex";
import Button from "@components/Button";
import SizedBox from "@components/SizedBox";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import Progressbar from "@components/Progressbar";
import CircularProgressbar from "@src/components/CircularProgressbar";

interface IProps {}

const Root = styled.div`
  width: 100%;
  display: grid;
  @media (min-width: 880px) {
    grid-template-columns: 3fr 1fr;
    column-gap: 20px;
  }
`;
const TextContainer = styled.div`
  display: grid;
  grid-template-columns: 3fr 3fr 3fr;
  align-items: end;
  row-gap: 24px;

  & > * {
    :first-of-type {
      padding-left: 0;
    }

    :last-of-type {
      padding-right: 0;
      align-items: flex-end;

      > div {
        justify-content: flex-end;
      }
    }
  }

  .large {
    font-size: 48px;
    line-height: 48px;
    @media (min-width: 880px) {
      font-size: 64px;
      line-height: 64px;
      //border-right: 0.5px solid #dfe5fa;
    }
  }

  .middle {
    font-size: 24px;
    line-height: 32px;

    @media (min-width: 880px) {
      font-size: 40px;
      line-height: 48px;
    }
  }

  .secondary {
    font-size: 14px;
    line-height: 24px;

    @media (min-width: 880px) {
      font-size: 18px;
    }
  }

  .small {
    //font-size: 14px;
    //line-height: 24px;

    @media (min-width: 880px) {
      font-size: 40px;
      line-height: 48px;
    }
  }
`;

const DashboardStats: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const generalData = [
    { title: "Supply balance", value: "$5,532.32", className: "large" },
    { title: "Daily reward", value: "$1,142.21", className: "middle" },
    { title: "USDC wallet balance", value: "$1,621.22", className: "small" },
  ];
  const userData = [
    { title: "Available to Borrow", value: "770", className: "middle" },
    { title: "Total debt", value: "$1,514.98", className: "middle" },
    { title: "Liquidation treshold", value: "60", className: "middle" },
  ];
  const handleSupplyUsdcClick = () => {
    vm.setAction("supply");
    vm.setActionTokenAssetId(TOKENS_BY_SYMBOL.USDC.assetId);
  };
  return (
    <Root>
      <TextContainer>
        {generalData.map(({ title, value, className }) => (
          <Column crossAxisSize="max" key={title}>
            <Text
              fitContent
              type="secondary"
              weight={600}
              className="secondary"
            >
              {title}
            </Text>
            <Text fitContent className={`${className}`} weight={600}>
              {value}
            </Text>
          </Column>
        ))}
        {/*{userData != null && (*/}
        <>
          <Column>
            <Text
              fitContent
              type="secondary"
              weight={600}
              className="secondary"
            >
              Available to borrow
            </Text>
            <Text fitContent className="middle" weight={600}>
              770$
            </Text>
            <SizedBox height={14} />
            <Progressbar percent={60} />
          </Column>
          <Column>
            <Text
              fitContent
              type="secondary"
              weight={600}
              className="secondary"
            >
              Total debt
            </Text>
            <Text fitContent className="middle" weight={600}>
              $1,514.98
            </Text>
          </Column>
          <Column>
            <Text
              fitContent
              type="secondary"
              weight={600}
              className="secondary"
            >
              Liquidation treshold
            </Text>
            <Row alignItems="center">
              <CircularProgressbar percent={60} />
              <SizedBox width={4} />
              <Text fitContent className="middle" weight={600}>
                60%
              </Text>
            </Row>
          </Column>
        </>
        {/*)}*/}
      </TextContainer>
      <Row crossAxisSize="max" alignItems="flex-end">
        <Button onClick={handleSupplyUsdcClick} fixed>
          Supply USDC
        </Button>
        <SizedBox width={8} />
        <Button disabled fixed>
          Borrow USDC
        </Button>
      </Row>
    </Root>
  );
};
export default observer(DashboardStats);
