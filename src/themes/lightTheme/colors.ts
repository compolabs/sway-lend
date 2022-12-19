const colors = {
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
