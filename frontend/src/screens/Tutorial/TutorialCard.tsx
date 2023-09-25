import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";

interface IProps {
  title: string;
  complexity: string;
  time: string;
  pic: string;
  link?: string;
}

const Root = styled.div`
  //width: 100%;
  display: flex;
  border-radius: 4px;
  flex-direction: column;
  padding: 16px;
  background: ${({ theme }) => theme.colors.neutral5};
`;
const Pic = styled.div`
  height: 124px;
  border-radius: 4px;
  padding: 16px;
  background: ${({ theme }) => theme.colors.neutral4};
`;
const TutorialCard: React.FC<IProps> = ({ title, complexity, time, pic }) => {
  return (
    <Root>
      <Pic />
      <SizedBox height={12} />
      <Text size="medium" weight={700}>
        {title}
      </Text>
      <SizedBox height={12} />
      <Row justifyContent="space-between">
        <Text type="secondary" weight={600} fitContent>
          Complexity
        </Text>
        <Text weight={600} fitContent>
          {complexity}
        </Text>
      </Row>
      <SizedBox height={12} />
      <Row justifyContent="space-between">
        <Text type="secondary" weight={600} fitContent>
          Average time
        </Text>
        <Text weight={600} fitContent>
          {time}
        </Text>
      </Row>
      <SizedBox height={12} />
      <Button fixed>Start tutorial</Button>
    </Root>
  );
};
export default TutorialCard;
