import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import { CircularProgressbar as Bar } from "react-circular-progressbar";
import "react-circular-progressbar/dist/styles.css";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  percent: number;
}

const Root = styled.div`
  width: 32px;
  height: 32px;
  border-radius: 50%;

  .CircularProgressbar .CircularProgressbar-trail {
    stroke: ${({ theme }) => theme.colors.progressBar.red};
    stroke-width: 8px;
  }

  .CircularProgressbar .CircularProgressbar-path {
    stroke-width: 8px;
    stroke: ${({ theme }) => theme.colors.progressBar.main};
  }

  .CircularProgressbar .CircularProgressbar-text {
    fill: ${({ theme }) => theme.colors.success500};
  }

  .CircularProgressbar.CircularProgressbar-inverted .CircularProgressbar-trail {
    stroke: ${({ theme }) => theme.colors.white};
  }
`;

const CircularProgressbar: React.FC<IProps> = ({ percent, ...rest }) => {
  return (
    <Root {...rest}>
      <Bar value={percent} />
    </Root>
  );
};
export default CircularProgressbar;
