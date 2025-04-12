import IconCheckBox from "@s/iconCheckBox/IconCheckBox.tsx";
import IconChecked from "@i/IconChecked.tsx";
import IconBox from "@i/IconBox.tsx";

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