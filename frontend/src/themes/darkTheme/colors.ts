const colors = {
  neutral2: "#E3E9F9",
  neutral3: "#DFE5FA",
  neutral4: "#9A9ABD",
  neutral5: "#313a45",
  neutral6: "#1F262B",
  neutral7: "#1A1D1F",
  primary01: "#3FE8BD",

  primary800: "#363870",
  primary650: "#8082C5",
  primary300: "#C6C9F4",
  primary100: "#F1F2FE",
  primary50: "#f8f8ff",
  blue500: "#7075E9",
  success: "#35A15A",
  success550: "#1F8943",
  success500: "#35A15A",
  success100: "#E6F3EB",
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
  primary800: "#FFFFFF",
  primary650: "#C6C9F4",

  primary300: "#8182C5",
  primary100: "#363970",
  primary50: "#181931",
  white: "#232345",
  success100: "#1C2F37",
  error100: "#3A2A3D",
  attention100: "#3A303F",

  mainBackground: colors.neutral7,
  text: colors.neutral2,
  disabledBtnTextColor: "rgba(255, 255, 255, 0.35)",
  disabledBtnColor: colors.neutral5,

  divider: colors.neutral3,
  switch: {
    background: colors.neutral5,
    circleColor: colors.neutral6,
  },
  modal: {
    background: colors.neutral6,
    border: colors.neutral5,
    loginTypeBackground: colors.neutral5,
  },
  table: {
    headerColor: colors.neutral4,
    background: "rgba(169, 213, 245, 0.05);",
  },
  tooltip: {
    border: "none",
    background: "#262F33",
    hoverElement: colors.neutral5,
  },
  header: {
    navLinkBackground: colors.neutral5,
    walletInfoColor: colors.neutral4,
    walletAddressBackground: colors.neutral5,
  },
  skeleton: {
    base: colors.neutral6,
    highlight: colors.neutral5,
  },
  //todo change disabled to selected
  dashboard: {
    tokenRowColor: "#313A45",
    tokenRowDisabled: "#262F33",
    cardBackground: "#262F33",
  },
  supplyBtn: {
    background: colors.neutral6,
    backgroundSelected: colors.neutral5,
  },

  card: {
    background: "rgba(169, 213, 245, 0.05);",
    border: colors.neutral5,
  },
  icon: {
    borderColor: colors.primary800,
  },
  gradient: "rgba(0, 0, 0, 0.5);",
  tokenDescGradient:
    "linear-gradient(180deg, rgba(248, 248, 255, 0) 0%, #181931 100%)",
  noNftGradient:
    "-webkit-linear-gradient(rgba(255, 255, 255, 0), rgb(20 22 49) 57.65%);",
};
