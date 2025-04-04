import React from "react";
import VLabel from "./VerticalLabel/VLabel.tsx";
import {Textarea} from "../../@shadcn/components/ui/textarea.tsx";

export interface InputVLProps extends React.InputHTMLAttributes<HTMLTextAreaElement> {
  label: string;
  hoverText?: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
export default function TextareaVL({hoverText, label, ...props}: InputVLProps) {
  return <VLabel name={label} hoverText={hoverText}>
    <Textarea {...props}/>
  </VLabel>;
}