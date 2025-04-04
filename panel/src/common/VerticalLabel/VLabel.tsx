import {Label} from "../../../@shadcn/components/ui/label.tsx";
import React from "react";
import {Tooltip, TooltipContent, TooltipTrigger} from "../../../@shadcn/components/ui/tooltip.tsx";

export interface VLabelProps {
  name: string,
  hoverText?: string,
  children: React.ReactNode
}

export default function VLabel({name, children, hoverText}: VLabelProps) {
  if (hoverText == undefined) {
    return <Label>
      <span style={{width: "100%", display: "flex", flexDirection: "row"}}>{name}</span>
      <div className="labelChildContainer" style={{paddingTop: "0.5rem"}}>
        {children}
      </div>
    </Label>
  }

  return <Label>
    <Tooltip>
      <TooltipContent>
        <span style={{
          fontSize: "medium",
          maxWidth: "40em",
          display: "block",
          lineHeight: "normal",
          padding: "5px",
        }}>{hoverText}</span>
      </TooltipContent>
      <TooltipTrigger>
        <span style={{width: "100%", display: "flex", flexDirection: "row"}}>{name}</span>
      </TooltipTrigger>
    </Tooltip>
    <div className="labelChildContainer" style={{paddingTop: "0.5rem"}}>
      {children}
    </div>
  </Label>
}