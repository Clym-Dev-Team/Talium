import {Textarea} from "@shadcn/textarea.tsx";
import "./TemplateEditor.css"
import {Popover, PopoverContent, PopoverTrigger} from "@shadcn/popover.tsx";
import {UseFormRegisterReturn} from "react-hook-form";

export interface TemplateEditorProps {
  varSchema: string,
  register: UseFormRegisterReturn<string>
}

function getVarsFromJson(varJsonSchema: string) {
  try {
    return Object.entries(JSON.parse(varJsonSchema));
  } catch (e) {
    return [];
  }
}

export default function TemplateEditor({varSchema, register}: TemplateEditorProps) {
  const vars = getVarsFromJson(varSchema);
  return <div className="templateEditor">
    <div className="templateVars">
      {vars.map((tVar) => <Popover>
        <PopoverTrigger>{tVar[0]}</PopoverTrigger>
        <PopoverContent
          className="dark popoverContent">{JSON.stringify(tVar[1], null, 4).replaceAll("\"", "")}</PopoverContent>
      </Popover>)}
    </div>
    <div className="templateBody">
      <Textarea placeholder={"Enter Template here"} {...register} />
    </div>
  </div>
}