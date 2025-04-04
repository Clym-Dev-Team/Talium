import {Input} from "../../@shadcn/components/ui/input.tsx";
import React from "react";
import VLabel from "./VerticalLabel/VLabel.tsx";

export interface InputVLProps extends React.InputHTMLAttributes<HTMLInputElement> {
  i18nFieldId: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
export default function InputVL({i18nFieldId, ...props}: InputVLProps) {
  return <VLabel i18nFieldId={i18nFieldId}>
    <Input {...props}/>
  </VLabel>;
}