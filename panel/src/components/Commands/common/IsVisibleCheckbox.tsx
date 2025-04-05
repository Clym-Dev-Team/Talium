import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";
import IconHidden from "@/assets/IconHidden.tsx";
import IconList from "@/assets/IconList.tsx";

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