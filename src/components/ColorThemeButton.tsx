import { setTheme } from "@tauri-apps/api/app"
import type { Theme } from "@tauri-apps/api/window"
import { useState } from "react"
import ActionButton from "./ActionButton"

export default function ColorThemeButton() {
  const [theme, setThemeState] = useState<Theme>("light")

  const toggleTheme = async () => {
    setThemeState(theme === "light" ? "dark" : "light")
    await setTheme(theme)
    document.body.classList.toggle("dark")
  }

  return (
    <ActionButton onClick={toggleTheme}>
      {theme === "light" ? "Dark Mode" : "Light Mode"}
    </ActionButton>
  )
}
