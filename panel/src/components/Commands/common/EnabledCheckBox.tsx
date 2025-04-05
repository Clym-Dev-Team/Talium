import IconPowerOn from "@/assets/IconPowerOn.tsx";
import IconPowerOff from "@/assets/IconPowerOff.tsx";
import IconCheckBox from "@/common/IconCheckBox/IconCheckBox.tsx";

export interface EnabledCheckBoxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  hoverText?: string
}

export default function EnabledCheckBox(props: EnabledCheckBoxProps) {
  return <IconCheckBox
    checked={props.checked}
    onChange={props.onChange}
    hoverText={props.hoverText}
    icon={<IconPowerOff/>}
    checkedIcon={<IconPowerOn/>}
  />;

}