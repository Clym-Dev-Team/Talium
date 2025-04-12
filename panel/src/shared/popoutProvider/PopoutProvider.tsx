import React, {createContext, useCallback, useContext, useState} from "react";
import "./PopoutProvider.css"

type PopoutContextType = {
  showPopout: (component: React.ReactNode) => void;
  clearPopout: () => void;
};

const PopoutContext = createContext<PopoutContextType>({
  showPopout: (_: React.ReactNode) => {},
  clearPopout: () => {},
});

interface PopoutProps {
  children: React.ReactNode;
}

export const PopoutProvider = ({children}: PopoutProps) => {
  const [component, setComponent] = useState<React.ReactNode | null>(null);

  const showPopout = useCallback((comp: React.ReactNode) => {
    setComponent(comp);
  }, []);

  const clearPopout = useCallback(() => {
    setComponent(null);
  }, []);

  return <PopoutContext.Provider value={{showPopout, clearPopout}}>
    {children}
    {component && (
      <div className="popoutWrapContainer">
        {component}
      </div>
    )}
  </PopoutContext.Provider>;
};

export const usePopout = () => useContext(PopoutContext);
