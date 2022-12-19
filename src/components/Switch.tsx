import styled from "@emotion/styled";
import React from "react";

interface IProps {
  value: boolean;
  onChange: () => void;
}

const Root = styled.div`
  display: flex;
  flex-direction: column;

  .switch {
    position: relative;
    display: inline-block;
    width: 40px;
    height: 24px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: ${({ theme }) => theme.colors.primary100};

    :disabled {
      background-color: ${({ theme }) => theme.colors.primary300};
    }

    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    width: 18px;
    height: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    box-shadow: 0px 2px 4px rgba(54, 56, 112, 0.12);

    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  input:checked + .slider {
    background-color: ${({ theme }) => theme.colors.blue500};
  }

  input:focus + .slider {
    box-shadow: 0 0 1px ${({ theme }) => theme.colors.primary100};
  }

  input:checked + .slider:before {
    -webkit-transform: translateX(16px);
    -ms-transform: translateX(16px);
    transform: translateX(16px);
  }

  .slider.round {
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(54, 56, 112, 0.12);
  }

  .slider.round:before {
    border-radius: 6px;
  }
`;

const Switch: React.FC<IProps> = ({ value, onChange }) => {
  return (
    <Root>
      <label className="switch">
        <input type="checkbox" checked={value} onChange={onChange} />
        <span className="slider round" />
      </label>
    </Root>
  );
};
export default Switch;
