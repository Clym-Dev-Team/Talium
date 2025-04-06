import React, {createContext, useCallback, useContext, useState} from "react";
import "./PopoutProvider.css"

type ComponentContextType = {
  showComponent: (component: React.ReactNode) => void;
  clearComponent: () => void;
};

const ComponentContext = createContext<ComponentContextType>({
  showComponent: (_: React.ReactNode) => {},
  clearComponent: () => {},
});

export const PopoutProvider = ({children}) => {
  const [component, setComponent] = useState<React.ReactNode | null>(null);

  const showComponent = useCallback((comp: React.ReactNode) => {
    setComponent(comp);
  }, []);

  const clearComponent = useCallback(() => {
    setComponent(null);
  }, []);

  return <ComponentContext.Provider value={{showComponent, clearComponent}}>
    {children}
    {component && (
      <div className="popoutWrapContainer">
        {component}
      </div>
    )}
  </ComponentContext.Provider>;
};

export const useComponentOverlay = () => useContext(ComponentContext);
