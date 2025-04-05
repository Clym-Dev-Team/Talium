import React from "react";
import {Input} from "@shadcn/input.tsx";
import VLabel from "./VLabel.tsx";

export interface InputVLProps extends React.InputHTMLAttributes<HTMLInputElement> {
  i18nFieldId: string;
}

// Input with Vertical label (label positioned vertically above the Input) (and with an optional info message on hover)
const InputVL = React.forwardRef<HTMLInputElement, InputVLProps>(({i18nFieldId, ...rest}, ref) => {
  return <VLabel i18nFieldId={i18nFieldId}>
    <Input ref={ref} {...rest}/>
  </VLabel>;
});

export default InputVL;