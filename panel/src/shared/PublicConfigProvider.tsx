import React, {createContext, useEffect, useState} from "react";

export const ENV_TWITCH_CLIENT_ID: string = import.meta.env.VITE_TWITCH_CLIENT_ID

interface PublicConfig {
  panelBaseUrl: string;
  serverBaseUrl: string;
  twitchClientId: string;
}

const PublicConfigContext = createContext<PublicConfig>({
  panelBaseUrl: "",
  serverBaseUrl: "",
  twitchClientId: "",
})

interface PublicConfigProps {
  children: React.ReactNode;
}

export default function PublicConfigProvider({children}: PublicConfigProps) {
  const [config, setConfig] = useState<PublicConfig | undefined>()

  useEffect(() => {
    let isMounted = true;
    fetch(location.origin + "/public.properties")
      .then((response) => response.text())
      .then((value) => {
        const config = parsePropertiesToMap(value)
        var panelBaseUrl = config.get("panelBaseUrl");
        var serverBaseUrl = config.get("serverBaseUrl");
        var twitchClientId = config.get("twitchClientId");

        if (!panelBaseUrl) {
          console.error("No panelBaseUrl in config!");
          panelBaseUrl = location.origin;
        }
        if (!serverBaseUrl) {
          console.error("No serverBaseUrl in config!");
          serverBaseUrl = location.origin + "/backend";
        }
        if (!twitchClientId || serverBaseUrl.trim() === "") {
          if (!ENV_TWITCH_CLIENT_ID) {
            console.error("No twitchClientId in config!");
            twitchClientId = "";
          }
          twitchClientId = ENV_TWITCH_CLIENT_ID;
        }
        if (isMounted) setConfig({
          panelBaseUrl: panelBaseUrl!,
          serverBaseUrl: serverBaseUrl!,
          twitchClientId: twitchClientId!,
        });
      })
      .catch((error) => {
        console.error("Error loading public Config: ", error);
      })
    return () => {
      isMounted = false;
    }
  }, []);

  return <PublicConfigContext.Provider value={config!}>
    {children}
  </PublicConfigContext.Provider>
}

function parsePropertiesToMap(content: string): Map<string, string> {
  const map = new Map<string, string>();

  const lines = content.split('\n');
  for (let line of lines) {
    line = line.trim();

    // Skip empty lines and comments
    if (line === '' || line.startsWith('#') || line.startsWith('!')) {
      continue;
    }

    const separatorIndex = line.indexOf('=');
    if (separatorIndex === -1) {
      console.log("Malformed property file, line: ", line);
      continue;
    }

    const key = line.substring(0, separatorIndex).trim();
    const value = line.substring(separatorIndex + 1).trim();

    map.set(key, value);
  }

  return map;
}
