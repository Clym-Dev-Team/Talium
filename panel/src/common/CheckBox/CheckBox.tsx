import IconBox from "@/assets/IconBox.tsx";
import IconChecked from "@/assets/IconChecked.tsx";
import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";

export interface CheckBoxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  hoverText?: string
}

export default function CheckBox(props: CheckBoxProps) {
  return <IconCheckBox
    checked={props.checked}
    onChange={props.onChange}
    hoverText={props.hoverText}
    icon={<IconBox/>}
    checkedIcon={<IconChecked/>}
  />;

}