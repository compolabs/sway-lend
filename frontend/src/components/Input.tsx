import styled from "@emotion/styled";
import React, { ChangeEvent, useState } from "react";
import Text from "@components/Text";

interface IProps
  extends Omit<
    React.DetailedHTMLProps<
      React.InputHTMLAttributes<HTMLInputElement>,
      HTMLInputElement
    >,
    "onChange" | "prefix"
  > {
  value?: string;
  onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
  suffix?: JSX.Element;
  prefix?: JSX.Element;
  suffixCondition?: boolean;
  error?: boolean;
  errorText?: string;
  description?: string;
}

const Root = styled.div<{ focused?: boolean; error?: boolean }>`
  width: 100%;
  background: none;

  align-items: center;
  justify-content: space-between;
  display: flex;
  box-sizing: border-box;
  height: 48px;

  input {
    padding: 0;
    width: 100%;
    color: ${({ focused, theme }) =>
      focused ? `${theme.colors.text}` : `${theme.colors.text}`};
    outline: none;
    border: none;

    ::placeholder {
      color: ${({ theme }) => `${theme.colors.text}`};
    }
    caret-color: ${({ theme }) => theme.colors.primary01};
    font-weight: 600;
    line-height: 32px;
    background: transparent;
    color: ${({ theme }) => theme.colors.text};

    ::-webkit-outer-spin-button,
    ::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }

    [type="number"] {
      -moz-appearance: textfield;
    }

    ::placeholder {
      color: ${({ theme }) => theme.colors.text};
    }
  }
`;

const Input: React.FC<IProps> = ({
  value,
  onChange,
  prefix,
  suffix,
  suffixCondition,
  placeholder,
  error,
  errorText,
  description,
  ...rest
}) => {
  const [focused, setFocused] = useState(false);
  return (
    <>
      <Root focused={focused} error={error} {...rest}>
        {prefix && prefix}
        <input
          onChange={onChange}
          value={value}
          placeholder={placeholder}
          onFocus={() => setFocused(true)}
          onBlur={() => setFocused(false)}
        />
        {suffixCondition || (suffix != null && suffix)}
      </Root>
      {description && (
        <Text size="small" type="secondary" style={{ paddingTop: 4 }}>
          {description}
        </Text>
      )}
    </>
  );
};
export default Input;
