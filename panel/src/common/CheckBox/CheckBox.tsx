import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";
import IconChecked from "@/assets/IconChecked.tsx";
import IconBox from "@/assets/IconBox.tsx";

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