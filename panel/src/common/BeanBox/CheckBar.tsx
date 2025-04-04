import React from "react";
import {Property} from "csstype";
import {Tooltip, TooltipContent, TooltipTrigger} from "../../../@shadcn/components/ui/tooltip.tsx";
import {Label} from "@radix-ui/react-label";
import IconX from "../../assets/IconX.tsx";
import IconCheck from "../../assets/IconCheck.tsx";
import "./BeanCheckBox.css"

export interface CheckBarProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  hoverText?: string,
  maxWidth?: Property.MaxWidth,
  children: React.ReactNode
}

export default function CheckBar(props: CheckBarProps) {
  return <Tooltip>
    {props.hoverText ? <TooltipContent>{props.hoverText}</TooltipContent> : ""}
    <TooltipTrigger onKeyDown={event => {
      if (event.key === ' ' || event.key === 'Enter') {
        event.stopPropagation();
        props.onChange(!props.checked)
      }
    }}>
      <Label className={props.checked ? "beanCheckBox checked" : "beanCheckBox"} style={{maxWidth: props.maxWidth}} onClick={event => {
        event.stopPropagation();
        props.onChange(!props.checked)
      }}>
        <span className="backGround"/>
        <span className="content">{props.checked ? <IconCheck/> : <IconX/>}{props.children}</span>
      </Label>
    </TooltipTrigger>
  </Tooltip>;
}