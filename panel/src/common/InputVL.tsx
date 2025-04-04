import {Input} from "../../@shadcn/components/ui/input.tsx";
import React from "react";
import VLabel from "./VerticalLabel/VLabel.tsx";

export interface InputVLProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  hoverText?: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
export default function InputVL({hoverText, label, ...props}: InputVLProps) {
  return <VLabel name={label} hoverText={hoverText}>
    <Input {...props}/>
  </VLabel>;
}