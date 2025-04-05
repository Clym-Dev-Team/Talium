import {Label} from "@shadcn/label.tsx";
import {Tooltip, TooltipContent, TooltipTrigger} from "@shadcn/tooltip.tsx";
import {useTranslation} from "react-i18next";
import "./InfoData.css"

export interface InfoDataProps {
  i18nFieldId: string,
  data: string
}

export default function InfoData({i18nFieldId, data}: InfoDataProps) {
  const {t, i18n} = useTranslation();
  let tooltipKey = i18nFieldId + ".tooltip";

  if (!i18n.exists(tooltipKey)) {
    return <Label className="infoDataLabel">
      <div className="infoDataLabel">
        {t(i18nFieldId + ".label")}: {data}
      </div>
    </Label>
  }

  return <Label className="infoDataLabel">
    <Tooltip>
      <TooltipContent>
        <span style={{
          fontSize: "medium",
          fontWeight: "normal",
          maxWidth: "40em",
          display: "block",
          lineHeight: "normal",
          padding: "5px",
        }}>{t(tooltipKey)}</span>
      </TooltipContent>
      <TooltipTrigger style={{ cursor: "default" }}>
        <span>{t(i18nFieldId + ".label")}: </span>
      </TooltipTrigger>
    </Tooltip>
    <div>
      {data}
    </div>
  </Label>
}