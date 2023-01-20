import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import { Column, Row } from "@src/components/Flex";
import Text from "@src/components/Text";
import SizedBox from "@components/SizedBox";
import Divider from "@src/components/Divider";
import useWindowSize from "@src/hooks/useWindowSize";

interface IProps {}

const Root = styled.div`
  width: 100%;
  display: grid;
  align-items: flex-end;

  .main-data {
    font-size: 48px;
    line-height: 48px;
  }

  .data {
    font-size: 24px;
    line-height: 32px;
  }

  .title {
    font-size: 14px;
    line-height: 24px;
  }

  @media (min-width: 880px) {
    grid-template-columns: 2fr 1fr;
    .main-data {
      font-size: 64px;
      line-height: 64px;
    }

    .data {
      font-size: 40px;
      line-height: 48px;
    }

    .title {
      font-size: 18px;
      line-height: 24px;
    }
  }
`;
const DashboardStats: React.FC<IProps> = () => {
  const { width } = useWindowSize();

  return (
    <Root>
      {width && width >= 880 ? (
        <>
          <Row justifyContent="space-between" alignItems="center">
            <Column crossAxisSize="max">
              <Text className="title" type="secondary" weight={600}>
                Supply balance
              </Text>
              <Text className="main-data" size="large" weight={600}>
                $0.00
              </Text>
            </Column>
            <Column crossAxisSize="max">
              <Text className="title" type="secondary" weight={600}>
                APY
              </Text>
              <Text className="data" size="big" weight={600}>
                32.54%
              </Text>
            </Column>
          </Row>
          <Column crossAxisSize="max">
            <Text
              className="title"
              textAlign="end"
              type="secondary"
              weight={600}
            >
              Borrow balance
            </Text>
            <Text textAlign="end" className="data" size="big" weight={600}>
              $1,000,514.63
            </Text>
          </Column>
        </>
      ) : (
        <>
          <Column>
            <Text className="title" type="secondary" weight={600}>
              Supply balance
            </Text>
            <Text className="main-data" size="big" weight={600}>
              $0.00
            </Text>
          </Column>
          <SizedBox height={16} />
          <Divider />
          <SizedBox height={16} />
          <Row alignItems="center" justifyContent="space-between">
            <Column>
              <Text className="title" type="secondary" weight={600}>
                APY
              </Text>
              <Text className="data" weight={600}>
                32.54%
              </Text>
            </Column>
            <Column style={{ textAlign: "end" }}>
              <Text className="title" type="secondary" weight={600}>
                Borrow balance
              </Text>
              <Text className="data" size="medium" weight={600}>
                $0.00
              </Text>
            </Column>
          </Row>
        </>
      )}
    </Root>
  );
};
export default observer(DashboardStats);
