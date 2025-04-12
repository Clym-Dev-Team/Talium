import "./InfoBox.css"
import IconInfoS from "@i/IconInfoS.tsx";

export interface WarningBoxProps {
  children: React.ReactNode;
}

export default function InfoBox({children}: WarningBoxProps) {
  return <div className="warningBoxWrapper">
    <div className="warningBox">
      <div className="iconBox">
        <IconInfoS/>
      </div>
      <div className="body">
        <div className="title"><h1>Info</h1></div>
        {children}
      </div>
    </div>
  </div>
}