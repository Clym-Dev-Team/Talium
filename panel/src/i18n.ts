import i18next from "i18next";
import en from "@/../locales/en.json";

i18next.init({
  lng: "en", // Default language
  fallbackLng: "en",
  debug: false,
  resources: {
    en: { translation: en },
  },
});

export default i18next;
