import React from "react";
import VLabel from "./VLabel.tsx";
import {Textarea} from "@shadcn/textarea.tsx";

export interface TextAreaVLProps extends React.InputHTMLAttributes<HTMLTextAreaElement> {
  i18nFieldId: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
const TextareaVL = React.forwardRef<HTMLTextAreaElement, TextAreaVLProps>((props, ref) => {
  return <VLabel i18nFieldId={props.i18nFieldId}>
    <Textarea ref={ref} {...props}/>
  </VLabel>;
});

export default TextareaVL;