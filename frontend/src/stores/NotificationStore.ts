import { makeAutoObservable } from "mobx";
import RootStore from "@stores/RootStore";
import { THEME_TYPE } from "@src/themes/ThemeProvider";
import { Theme, toast } from "react-toastify";
import { ToastOptions } from "react-toastify/dist/types";
import getAlert from "@src/utils/alertUtil";
import axios from "axios";

export type TNotifyOptions = ToastOptions & {
  link?: string;
  linkTitle?: string;
  title?: string;
};

class NotificationStore {
  public rootStore: RootStore;

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
  }

  toast(content: string, options: TNotifyOptions = {}) {
    let theme: Theme =
      this.rootStore.settingsStore.selectedTheme === THEME_TYPE.DARK_THEME
        ? "dark"
        : "light";

    toast(getAlert(content, options) ?? content, {
      ...options,
      theme,
    });

    const data = JSON.stringify({
      collection: "logs",
      database: "sway-lend",
      dataSource: "Cluster0",
      document: {
        address: this.rootStore.accountStore.address,
        action: options.title,
        content: content,
      },
    });

    console.log(process.env.REACT_APP_LOG_DB_API_KEY);

    const config = {
      method: "post",
      url: "https://data.mongodb-api.com/app/data-amrrc/endpoint/data/v1/action/insertOne",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
        "api-key": process.env.REACT_APP_LOG_DB_API_KEY,
      },
      data: data,
    };
    if (options.type === "error") {
      axios(config)
        .then((response) => console.log(JSON.stringify(response.data)))
        .catch((error) => console.log(error));
    }
  }
}

export default NotificationStore;
