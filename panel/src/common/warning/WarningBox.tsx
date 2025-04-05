import "./warningBox.css"
import IconWarning from "@/assets/IconWarning.tsx";

export interface WarningBoxProps {
  children: React.ReactNode;
}

export default function WarningBox({children}: WarningBoxProps) {
  return <div className="warningBoxWrapper">
    <div className="warningBox">
      <div className="warningTitle"><IconWarning/>Warning</div>
      {children}
    </div>
  </div>
}