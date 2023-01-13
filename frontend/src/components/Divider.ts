import styled from "@emotion/styled";

const Divider = styled.div`
  width: 100%;
  height: 1px;
  opacity: 0.2;
  background: ${({ theme }) => theme.colors.divider};
`;
export default Divider;
