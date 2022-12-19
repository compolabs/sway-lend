import React from "react";
import { css, Global, useTheme } from "@emotion/react";

const globalModalStyles = (theme: any) => `

body {
    margin: 0;
    font-family: "Roboto", sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    background:  ${theme.colors.white};
}

.rc-dialog-mask {
    background: rgba(0, 0, 0, 0.4);
}

.rc-dialog-wrap {
    flex-direction: column;
    align-items: center;
    justify-content: center;
    display: flex;
}

.rc-dialog {
    width: calc(100% - 32px);
}

.rc-dialog-content {
    background: ${theme.colors.white} ;
    border: 1px solid ${theme.colors.primary100} ;
    box-shadow: 0px 8px 56px rgba(54, 56, 112, 0.16);
    border-radius: 16px;
    overflow: hidden;
    min-height: 286px;
    padding: 0;
}

.rc-dialog-header {
    border-bottom: 1px solid ${theme.colors.primary100};
    background: ${theme.colors.white} ;
    padding: 16px 24px;
    max-height: 56px;
} 
.rc-dialog-header .send-asset{
    border-bottom: 1px solid ${theme.colors.primary100};
}

.rc-dialog-body {
    padding: 16px 24px 0 24px;
}

.rc-dialog-title {
    font-family: Roboto, sans-serif;
    font-style: normal;
    font-weight: 500;
    font-size: 16px;
    line-height: 24px;
    color: ${theme.colors.primary800};
}


.rc-dialog-close {
    opacity: 1;
}

.rc-notification {
    top: 30px;
    right: 16px;
}

.rc-notification-notice {
 background: ${theme.colors.white};
 
}

.rc-notification-notice-content {
    display: flex;
    background: ${theme.colors.white};
}

.custom-notification .rc-notification-notice-close {
    opacity: 1;
    top: 16px;
    right: 16px;
}

.custom-notification .rc-notification-notice-close > svg > path {
    fill: ${theme.colors.primary300};
}

.recharts-default-tooltip { 

background-color: ${theme.colors.white} !important;

}

.react-loading-skeleton {

--base-color: ${theme.colors.white};
--highlight-color: ${theme.colors.primary100};

}

.notifications-text {
 color: ${theme.colors.primary800};
}


`;

const GlobalStyles: React.FC = () => {
  const theme = useTheme();
  return <Global styles={css(globalModalStyles(theme))} />;
};

export default GlobalStyles;
