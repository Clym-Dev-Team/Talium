import {Template} from "./Template.ts";
import {useForm} from "react-hook-form";
import VLabel from "../../../common/VLabel.tsx";
import {Input} from "../../../../@shadcn/components/ui/input.tsx";
import TemplateEditor from "../common/templates/TemplateEditor.tsx";
import {SheetFooter} from "../../../../@shadcn/components/ui/sheet.tsx";
import {Button} from "../../../../@shadcn/components/ui/button.tsx";

export interface TemplateFormProps {
  template: Template,
  onSubmit: (template: Template) => void
  onDelete: (templateId: string) => void
}

export default function TemplateForm({template, onSubmit, onDelete}: TemplateFormProps) {
  const id = template.id;
  const {handleSubmit, register} = useForm<Template>({
    defaultValues: template,
  });

  function submit(template: Template) {
    template.id = id;
    onSubmit(template);
  }

  return <div className="commandPopup">
    <VLabel i18nFieldId="Internal Template Name/Id:">
      <Input id="templateId" type="text" {...register("id", {required: true, disabled: true})} />
    </VLabel>
    <VLabel i18nFieldId="Twitch Message Color:">
      <Input id="color" type="text" {...register("messageColor", {required: false})} />
    </VLabel>
    <VLabel i18nFieldId="Template:">
      <TemplateEditor varSchema={template.template} register={register("template")}/>
    </VLabel>
    <SheetFooter>
      <Button variant={"destructive"} onClick={() => onDelete(template.id)}>Delete</Button>
      <Button variant={"default"} onClick={handleSubmit(submit)}>Save</Button>
    </SheetFooter>
  </div>
}