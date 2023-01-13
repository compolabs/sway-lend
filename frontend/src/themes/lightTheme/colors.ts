const colors = {
  neutral1: "#EFF2FD",
  neutral2: "#E3E9F9",
  neutral3: "#DFE5FA",
  neutral5: "#313a45",
  neutral6: "#1F262B",
  neutral7: "#1A1D1F",
  primary01: "#3FE8BD",

  primary800: "#363870",
  primary650: "#8082C5",
  primary300: "#C6C9F4",
  primary100: "#F1F2FE",
  primary50: "#f8f8ff",
  white: "#FFFFFF",
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

  switch: {
    background: colors.neutral3,
  },
  header: {
    navLinkBackground: colors.neutral3,
  },

  card: {
    background: colors.white,
  },
  icon: {
    borderColor: colors.primary100,
  },
  gradient: "rgba(255, 255, 255, 0.5)",
  tokenDescGradient:
    "linear-gradient(180deg, rgba(248, 248, 255, 0) 0%, #f8f8ff 100%)",
  noNftGradient:
    "-webkit-linear-gradient(rgba(255, 255, 255, 0), rgba(241, 242, 254, 1) 57.65%);",
};
