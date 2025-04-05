import "./CommandEditSheet.css"
import {
  FieldArrayWithId,
  useFieldArray,
  UseFieldArrayRemove,
  UseFieldArrayUpdate,
  useForm,
  UseFormRegister
} from "react-hook-form";
import {SheetFooter} from "@shadcn/sheet.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@shadcn/select.tsx";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import {Command, CooldownTypes} from "./Command.ts";
import TemplateEditor from "@c/Commands/common/templates/TemplateEditor.tsx";
import IsVisibleCheckBox from "@c/Commands/common/IsVisibleCheckbox.tsx";
import EnabledCheckBox from "@c/Commands/common/EnabledCheckBox.tsx";
import VLabel from "@s/VLabel.tsx";
import InputUnit from "@s/InputUnit/InputUnit.tsx";
import CheckBox from "@s/CheckBox.tsx";
import IconX from "@i/IconX.tsx";

export interface CommandFormProps {
  command: Command,
  isNew?: boolean
  onSubmit: (command: Command) => void
  onDelete: (commandId: string) => void
}

export function CommandForm({command, isNew, onSubmit, onDelete}: CommandFormProps) {
  const USERCOMMANDPREFIX = "userCommand.";

  const {handleSubmit, register, control, setValue, getValues} = useForm<Command>({
    defaultValues: command,
  });
  const {fields, append, update, remove} = useFieldArray({name: "patterns", control})

  function submit(formCommand: Command) {
    if (isNew) {
      formCommand.id = USERCOMMANDPREFIX + formCommand.id
      if (formCommand.template != null) {
        formCommand.template.id = formCommand.id;
      }
    } else {
      // if form field is disabled the value will not exist in the command object from the form
      formCommand.id = command.id
    }
    onSubmit(formCommand);
  }

  return <div className="commandPopup">
    <VLabel i18nFieldId="Internal Command Name/Id:">
      <Input id="commandId" type="text" {...register("id", {required: true, disabled: !isNew})} />
    </VLabel>

    <div className="triggers">
      Trigger:
      {fields.map((field, index) => TriggerInput(index, field, register, update, remove))}
      <Button variant="secondary" className="addTrigger" onClick={() => append({
        isRegex: false,
        isVisible: true,
        isEnabled: true,
        pattern: ""
      })}>Add a new Alias</Button>
    </div>

    <VLabel i18nFieldId="Required Permission Level:">
      <Select defaultValue={getValues().permission} onValueChange={value => setValue("permission", value)}>
        <SelectTrigger>
          <SelectValue placeholder="Select a Permission Level"/>
        </SelectTrigger>
        <SelectContent className="dark">
          <SelectItem value="EVERYONE">EVERYONE</SelectItem>
          <SelectItem value="VIP">VIP</SelectItem>
          <SelectItem value="MODERATOR">MODERATOR</SelectItem>
          <SelectItem value="OWNER">OWNER</SelectItem>
        </SelectContent>
      </Select>
    </VLabel>

    <div className="cooldown">
      <VLabel i18nFieldId="Global Cooldown:">
        <InputUnit unitType={CooldownTypes} unitFieldValue={getValues("globalCooldown.type")} onUnitChange={unitType => setValue("globalCooldown.type", unitType)} registerValue={register("globalCooldown.amount", {required: true, min: 0, valueAsNumber: true})}/>
      </VLabel>
      <VLabel i18nFieldId="User Cooldown:">
        <InputUnit unitType={CooldownTypes} unitFieldValue={getValues("userCooldown.type")} onUnitChange={unitType => setValue("userCooldown.type", unitType)} registerValue={register("userCooldown.amount", {required: true, min: 0, valueAsNumber: true})}/>
      </VLabel>
    </div>

    { command.template != null ?
    <VLabel i18nFieldId="Template:">
      <TemplateEditor varSchema={command.varJsonSchema} register={register("template.template")}/>
    </VLabel> : <div className="noTemplate">This Command doesn't have a template because it triggers a callback inside the Bot</div>
    }
    <SheetFooter>
      <Button variant={"destructive"} onClick={() => onDelete(command.id)}>Delete</Button>
      <Button variant={"default"} onClick={handleSubmit(submit)}>Save</Button>
    </SheetFooter>
  </div>
}


function TriggerInput(index: number, field: FieldArrayWithId<Command, "patterns">, register: UseFormRegister<Command>, update: UseFieldArrayUpdate<Command, "patterns">, remove: UseFieldArrayRemove) {
  return <div className="trigger" key={index}>
    <Button className="removeTriggerBtn" onClick={() => remove(index)}><IconX/></Button>
    <Input type="text" placeholder="Trigger Pattern" {...register(`patterns.${index}.pattern`, {required: true})}/>
    <CheckBox checked={field.isRegex} onChange={checked => update(index, {...field, isRegex: checked})}
              hoverText="Regex Trigger"/>
    <IsVisibleCheckBox checked={field.isVisible} onChange={checked => update(index, {...field, isVisible: checked})}
                       hoverText="Visible in Command List"/>
    <EnabledCheckBox checked={field.isEnabled} onChange={checked => update(index, {...field, isEnabled: checked})}
                     hoverText="Enabled"/>
  </div>
}