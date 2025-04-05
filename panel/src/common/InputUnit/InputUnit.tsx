import "./InputUnit.css"
import {Input} from "@shadcn/input.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@shadcn/select.tsx";
import {UseFormRegisterReturn} from "react-hook-form";

export interface InputUnitProps<EnumType extends object, UnitField extends string> {
  unitType: EnumType
  unitFieldValue: string
  onUnitChange: (unitType: string) => void
  registerValue: UseFormRegisterReturn<UnitField>
}

export default function InputUnit<EnumType extends object, UnitField extends string>({
                                                                        unitType,
                                                                        unitFieldValue,
                                                                        onUnitChange,
                                                                        registerValue
                                                                      }: InputUnitProps<EnumType, UnitField>) {
  return <div className="inputUnitWrapper">
    <Input type="number" className="inputUnitValue" {...registerValue}/>
    <Select defaultValue={unitFieldValue} onValueChange={onUnitChange}>
      <SelectTrigger className="inputUnitUnit"><SelectValue placeholder="Select a Unit for this value"/></SelectTrigger>
      <SelectContent className="dark">
        {Object.keys(unitType).filter(value => isNaN(Number(value))).map((variant) =>
          <SelectItem value={variant}>{variant}</SelectItem>
        )}
      </SelectContent>
    </Select>
  </div>
}