import React from "react";
import {Property} from "csstype";
import {Tooltip, TooltipContent, TooltipTrigger} from "../../../@shadcn/components/ui/tooltip.tsx";
import {Label} from "@radix-ui/react-label";
import IconX from "../../assets/IconX.tsx";
import IconCheck from "../../assets/IconCheck.tsx";
import "./BeanCheckBox.css"

export interface BeanCheckBoxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  hoverText?: string,
  maxWidth?: Property.MaxWidth,
  children: React.ReactNode
}

export default function BeanCheckBox(props: BeanCheckBoxProps) {
  return <Tooltip>
    {props.hoverText ? <TooltipContent>{props.hoverText}</TooltipContent> : ""}
    <TooltipTrigger>
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