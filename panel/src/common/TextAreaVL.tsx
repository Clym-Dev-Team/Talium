import React from "react";
import VLabel from "./VerticalLabel/VLabel.tsx";
import {Textarea} from "../../@shadcn/components/ui/textarea.tsx";

export interface InputVLProps extends React.InputHTMLAttributes<HTMLTextAreaElement> {
  i18nFieldId: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
export default function TextareaVL({i18nFieldId, ...props}: InputVLProps) {
  return <VLabel i18nFieldId={i18nFieldId}>
    <Textarea {...props}/>
  </VLabel>;
}