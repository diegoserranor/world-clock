import { createContext, useState, ReactNode, useContext } from "react";

type NavScreen = "clock-list" | "clock-add";

interface Navigation {
  screen: NavScreen;
  setScreen: (screen: NavScreen) => void;
}

const initialContext: Navigation = {
  screen: "clock-list",
  setScreen: (_) => {},
};

const NavigationContext = createContext(initialContext);

export function useNavigation() {
    return useContext(NavigationContext);
}

export function Navigation({
  clockList,
  clockAdd,
}: {
  clockList: ReactNode;
  clockAdd: ReactNode;
}) {
  const [screen, setScreen] = useState<NavScreen>("clock-list");

  function renderScreen() {
    switch (screen) {
      case "clock-list":
        return clockList;
      case "clock-add":
        return clockAdd;
      default:
        return null;
    }
  }

  return (
    <NavigationContext value={{ screen, setScreen }}>
      {renderScreen()}
    </NavigationContext>
  );
}

