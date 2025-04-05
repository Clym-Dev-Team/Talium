import React from "react";
import {Textarea} from "@shadcn/textarea.tsx";
import VLabel from "./VLabel.tsx";

export interface TextAreaVLProps extends React.InputHTMLAttributes<HTMLTextAreaElement> {
  i18nFieldId: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
const TextareaVL = React.forwardRef<HTMLTextAreaElement, TextAreaVLProps>(({i18nFieldId, ...rest}, ref) => {
  return <VLabel i18nFieldId={i18nFieldId}>
    <Textarea ref={ref} {...rest}/>
  </VLabel>;
});

export default TextareaVL;