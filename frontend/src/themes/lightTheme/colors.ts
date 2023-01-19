const colors = {
  white: "#ffffff",
  neutral1: "#EFF2FD",
  neutral2: "#E3E9F9",
  neutral3: "#DFE5FA",
  neutral4: "#9A9ABD",
  neutral5: "#313a45",
  neutral6: "#1F262B",
  neutral7: "#1A1D1F",
  primary01: "#3FE8BD",
  primary03: "#FF6A55",
  secondary1: "#00B493",

  //todo delete not used colors
  primary800: "#363870",
  primary650: "#8082C5",
  primary300: "#C6C9F4",
  primary100: "#F1F2FE",
  primary50: "#f8f8ff",
  blue500: "#7075E9",
  success: "#35A15A",
  error: "#ED827E",
  success550: "#1F8943",
  success500: "#35A15A",
  success100: "#ccebdc",
  error550: "#D66662",
  error500: "#ED827E",
  error100: "#FCF0EF",
  attention550: "#D9916E",
  attention500: "#EDAA8A",
  attention100: "#FCF4F1",
};
// eslint-disable-next-line
export default {
  ...colors,
  text: colors.neutral6,
  mainBackground: colors.neutral1,
  menuBackground: colors.neutral5,
  divider: "rgba(31, 38, 43, 0.2)",
  disabledBtnTextColor: "rgba(26, 29, 31, 0.35);",
  disabledBtnColor: "rgba(184, 189, 208, 0.5)",

  switch: {
    background: colors.neutral3,
    circleColor: colors.neutral1,
  },
  switchButtons: {
    selectedBackground: colors.neutral1,
    selectedColor: colors.neutral7,
    secondaryBackground: colors.neutral3,
    secondaryColor: colors.neutral4,
  },

  button: {
    primaryBackground: colors.primary01,
    primaryColor: colors.neutral6,
    secondaryBackground: colors.neutral3,
    secondaryColor: colors.neutral6,
  },
  header: {
    navLinkBackground: colors.neutral3,
    walletInfoColor: colors.neutral4,
    walletAddressBackground: colors.neutral3,
    walletBalanceBackground: colors.neutral4,
  },
  skeleton: {
    base: colors.neutral3,
    highlight: colors.neutral2,
  },
  card: {
    background: "rgba(255, 255, 255, 0.5);",
    border: colors.neutral2,
  },

  modal: {
    background: colors.white,
    mask: "rgba(227, 233, 249, 0.8)",
  },
  table: {
    headerColor: colors.neutral4,
    background: "rgba(255, 255, 255, 0.5)",
  },
  tooltip: {
    border: "none",
    background: "#F9FAFF",
    hoverElement: "rgba(223, 229, 250, 0.6)",
  },
  dashboard: {
    tokenRowColor: "#F9FAFF",
    tokenRowSelected: colors.white,
    cardBackground: "#F9FAFF",
  },
  supplyBtn: {
    background: colors.neutral3,
    backgroundSelected: colors.neutral3,
  },
  icon: {
    borderColor: colors.primary100,
  },
  gradient: "rgba(255, 255, 255, 0.5)",
  notifications: {
    boxShadow:
      "0px 0px 14px -4px rgba(31, 38, 42, 0.05), 0px 32px 48px -8px rgba(31, 38, 42, 0.1)",
    background: colors.white,
  },
  progressBar: {
    main: colors.secondary1,
    secondary: colors.neutral3,
    red: colors.primary03,
  },

  tokenDescGradient:
    "linear-gradient(180deg, rgba(248, 248, 255, 0) 0%, #f8f8ff 100%)",
  noNftGradient:
    "-webkit-linear-gradient(rgba(255, 255, 255, 0), rgba(241, 242, 254, 1) 57.65%);",
};
