import React from "react";
import Layout from "@components/Layout";
import styled from "@emotion/styled";
import SizedBox from "@components/SizedBox";

interface IProps {}

const Root = styled.div``;

const Tutorial: React.FC<IProps> = () => {
  return (
    <Layout>
      <SizedBox height={24} />
      <Root>
        <iframe
          title="Tutorial form"
          src="https://docs.google.com/forms/d/e/1FAIpQLScqNaQ-oH-gfSabxW7k5qfMLNttSwNucOdKRVJemL1bvy0Guw/viewform?embedded=true"
          width="640"
          height="1779"
          frameBorder={0}
          marginHeight={0}
          marginWidth={0}
        >
          Loading...
        </iframe>
      </Root>
    </Layout>
  );
};

export default Tutorial;
