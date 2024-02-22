const systemSettingDark = window.matchMedia("(prefers-color-scheme: dark)");
const localStorageTheme = localStorage.getItem("theme");
const toggleThemeButton = document.getElementById("toggle-theme");

const calculateSettingAsThemeString = ({ localStorageTheme, systemSettingDark }) => {
    if (localStorageTheme !== null) {
      return localStorageTheme;
    }
  
    if (systemSettingDark.matches) {
      return "dark";
    }
  
    return "light";
}

let currentThemeSetting = calculateSettingAsThemeString({ localStorageTheme, systemSettingDark });
localStorage.setItem("theme", currentThemeSetting);
document.documentElement.setAttribute("data-theme", currentThemeSetting);
toggleThemeButton.children[0].classList.replace(currentThemeSetting === "dark" ? "fa-moon" : "fa-lightbulb", currentThemeSetting === "dark" ? "fa-lightbulb" : "fa-moon");
toggleThemeButton.children[0].classList.replace(currentThemeSetting === "dark" ? "fa-solid" : "fa-regular", currentThemeSetting === "dark" ? "fa-regular" : "fa-solid");

const toggleTheme = () => {
    currentThemeSetting = currentThemeSetting === "dark" ? "light" : "dark";
    localStorage.setItem("theme", currentThemeSetting);
    document.documentElement.setAttribute("data-theme", currentThemeSetting);
    toggleThemeButton.children[0].classList.replace(currentThemeSetting === "dark" ? "fa-moon" : "fa-lightbulb", currentThemeSetting === "dark" ? "fa-lightbulb" : "fa-moon");
    toggleThemeButton.children[0].classList.replace(currentThemeSetting === "dark" ? "fa-solid" : "fa-regular", currentThemeSetting === "dark" ? "fa-regular" : "fa-solid");
}
