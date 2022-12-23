import styled from "@emotion/styled";

const GridTable = styled.div<{
  desktopTemplate?: string;
  mobileTemplate?: string;
}>`
  & .gridTitle {
    display: grid;
    column-gap: 8px;
    grid-template-columns: ${({ mobileTemplate }) =>
      mobileTemplate ?? "6fr 2fr 1fr"};
    font-weight: normal;
    font-size: 14px;
    line-height: 20px;
    color: ${({ theme }) => theme.colors.primary650};
    padding: 14px 16px;
    box-sizing: border-box;
    border-bottom: 1px solid ${({ theme }) => theme.colors.primary100};
    margin-bottom: 8px;
    @media (min-width: 880px) {
      grid-template-columns: ${({ desktopTemplate }) =>
        desktopTemplate ?? "6fr 2fr 1fr"};
      padding: 14px 24px;
    }
  }

  & .gridRow {
    max-width: 100%;
    cursor: pointer;
    display: grid;
    column-gap: 8px;
    grid-template-columns: ${({ mobileTemplate }) =>
      mobileTemplate ?? "6fr 2fr 1fr"};
    font-weight: normal;
    font-size: 14px;
    line-height: 20px;
    color: ${({ theme }) => theme.colors.primary650};
    box-sizing: border-box;
    margin: 0 16px;
    padding: 16px 0;
    border-bottom: 1px solid ${({ theme }) => theme.colors.primary100};
    @media (min-width: 880px) {
      grid-template-columns: ${({ desktopTemplate }) =>
        desktopTemplate ?? "6fr 2fr 1fr"};
      margin: 0 24px;
    }

    :last-of-type {
      border-bottom: none;
    }
  }
`;

export default GridTable;
