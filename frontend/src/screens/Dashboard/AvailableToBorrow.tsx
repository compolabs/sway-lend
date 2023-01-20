import React from "react";
import { useStores } from "@stores";
import { Column, Row } from "@components/Flex";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import Progressbar from "@components/Progressbar";

interface IProps {}

const AvailableToBorrow: React.FC<IProps> = () => {
  const { accountStore } = useStores();
  if (!accountStore.isLoggedIn) return null;
  return (
    <Column crossAxisSize="max">
      <Row justifyContent="space-between">
        <Text fitContent weight={600} type="secondary" size="small">
          Available to Borrow
        </Text>
        <Text fitContent weight={600} type="secondary" size="small">
          60%
        </Text>
      </Row>
      <SizedBox height={4} />
      <Progressbar percent={60} />
    </Column>
  );
};
export default AvailableToBorrow;
