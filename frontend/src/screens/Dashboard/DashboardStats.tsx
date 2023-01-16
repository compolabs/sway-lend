import styled from "@emotion/styled";
import React from "react";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";
import Text from "@components/Text";
import { Column, Row } from "@src/components/Flex";
import useWindowSize from "@src/hooks/useWindowSize";

interface IProps {}

const Root = styled.div`
  display: grid;
  grid-template-columns: 3fr 2fr 3fr 3fr;
  align-items: end;

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
  const data = [
    { title: "Supply balance", value: "$5,532.32", className: "large" },
    { title: "NET APY", value: "32.54%", className: "middle" },
    { title: "Daily reward", value: "$1,142.21", className: "middle" },
    { title: "USDC wallet balance", value: "$1,621.22", className: "small" },
  ];
  const { width } = useWindowSize();
  return (
    <Root>
      {data.map(({ title, value, className }) => (
        <Column crossAxisSize="max" key={title}>
          <Text fitContent type="secondary" weight={600} className="secondary">
            {title}
          </Text>
          <Text fitContent className={`${className}`} weight={600}>
            {value}
          </Text>
        </Column>
      ))}
    </Root>
  );
};
export default observer(DashboardStats);
