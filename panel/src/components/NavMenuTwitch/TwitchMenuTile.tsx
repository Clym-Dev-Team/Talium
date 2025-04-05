import "./TwitchMenuTile.css"
import React, {useEffect} from "react";
import {useLocation} from "react-router-dom";
import {Tooltip, TooltipContent, TooltipTrigger} from "@shadcn/tooltip.tsx";

export interface TwitchMenuTileProps {
  icon: React.ReactNode
  label: string;
  target: string
  allowChildren?: boolean
}

export default function TwitchMenuTile({icon, target, allowChildren, label}: TwitchMenuTileProps) {
  const {pathname} = useLocation();
  const [highlight, setHighlight] = React.useState(false);

  useEffect(() => {
    if (allowChildren == undefined || allowChildren) {
      setHighlight(pathname.startsWith(target));
    } else {
      setHighlight(pathname == target);
    }
  }, [pathname, target]);


  return <Tooltip>
    <TooltipContent>
      <span className="tooltipContent">{label}</span>
    </TooltipContent>
    <TooltipTrigger>
      <a href={target} className={"twitchMenuTile " + (highlight ? "highlight" : "")}>
        {icon}
      </a>
    </TooltipTrigger>
  </Tooltip>
}