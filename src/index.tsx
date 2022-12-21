import React from "react";
import "./index.css";
import ReactDOM from "react-dom/client";
import App from "./App";
import { HashRouter as Router } from "react-router-dom";
import "normalize.css";
import { RootStore, storesContext } from "@stores";
import { loadState, saveState } from "@src/utils/localStorage";
import { autorun } from "mobx";
import "rc-notification/assets/index.css";
import "react-perfect-scrollbar/dist/css/styles.css";
import "rc-dialog/assets/index.css";
import ThemeWrapper from "./themes/ThemeProvider";
import GlobalStyles from "@src/themes/GlobalStyles";

const initState = loadState();

const mobxStore = new RootStore(initState);
autorun(
  () => {
    console.dir(mobxStore);
    saveState(mobxStore.serialize());
  },
  { delay: 1000 }
);

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
  <React.StrictMode>
    <storesContext.Provider value={mobxStore}>
      <ThemeWrapper>
        <Router>
          <App />
        </Router>
        <GlobalStyles />
      </ThemeWrapper>
    </storesContext.Provider>
  </React.StrictMode>
);
