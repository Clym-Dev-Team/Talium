import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";
import IconList from "@/assets/IconList.tsx";
import IconHidden from "@/assets/IconHidden.tsx";

export interface IsVisibleCheckBoxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  hoverText?: string
}

export default function IsVisibleCheckBox(props: IsVisibleCheckBoxProps) {
  return <IconCheckBox
    checked={props.checked}
  onChange={props.onChange}
  hoverText={props.hoverText}
  icon={<IconHidden/>}
  checkedIcon={<IconList/>}
  />;

}