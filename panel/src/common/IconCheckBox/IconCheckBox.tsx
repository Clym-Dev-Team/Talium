import React from "react";
import "./IconCheckBox.css"
import {Tooltip, TooltipContent, TooltipTrigger} from "../../../@shadcn/components/ui/tooltip.tsx";

export interface IconCheckBoxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  icon: React.ReactNode;
  checkedIcon: React.ReactNode;
  hoverText?: string
}

export default function IconCheckBox({checked, checkedIcon, hoverText, icon, onChange}: IconCheckBoxProps) {
  return <Tooltip>
    { hoverText ? <TooltipContent>{hoverText}</TooltipContent> : ""}
    <TooltipTrigger>
      <div className="iconCheckBox" onClick={event => {
        event.stopPropagation();
        onChange(!checked)
      }}>
        {checked ? checkedIcon : icon}
      </div>
    </TooltipTrigger>
  </Tooltip>;
}