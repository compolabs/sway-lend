import React from "react";
import { ThemeProvider } from "@emotion/react";
import { useObserver } from "mobx-react-lite";

import darkTheme from "@src/themes/darkTheme";
import lightTheme from "@src/themes/lightTheme";
import { useStores } from "@stores";

export enum THEME_TYPE {
  LIGHT_THEME = "lightTheme",
  DARK_THEME = "darkTheme",
}

interface IProps {
  children: React.ReactNode;
}

export const themes = {
  darkTheme,
  lightTheme,
};
//todo fix
const ThemeWrapper: React.FC<IProps> = ({ children }) => {
  const { settingsStore } = useStores();
  const selectedTheme = useObserver(() => settingsStore.selectedTheme);
  return (
    <ThemeProvider theme={themes[selectedTheme]}>{children}</ThemeProvider>
  );
};

export default ThemeWrapper;
