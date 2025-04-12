import IconCheckBox from "@s/iconCheckBox/IconCheckBox.tsx";
import IconPowerOn from "@i/IconPowerOn.tsx";
import IconPowerOff from "@i/IconPowerOff.tsx";

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