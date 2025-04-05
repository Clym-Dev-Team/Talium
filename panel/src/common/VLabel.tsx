import {Label} from "@shadcn/label.tsx";
import React from "react";
import {Tooltip, TooltipContent, TooltipTrigger} from "@shadcn/tooltip.tsx";
import {useTranslation} from "react-i18next";

export interface VLabelProps {
  i18nFieldId: string,
  children: React.ReactNode
}

export default function VLabel({i18nFieldId, children}: VLabelProps) {
  const {t, i18n} = useTranslation();
  let tooltipKey = i18nFieldId + ".tooltip";

  if (!i18n.exists(tooltipKey)) {
    return <Label>
      <span style={{width: "100%", display: "flex", flexDirection: "row"}}>{t(i18nFieldId + ".label")}</span>
      <div style={{paddingTop: "0.5rem"}}>
        {children}
      </div>
    </Label>
  }

  return <Label>
    <Tooltip>
      <TooltipContent>
        <span style={{
          fontSize: "medium",
          maxWidth: "40em",
          display: "block",
          lineHeight: "normal",
          padding: "5px",
        }}>{t(tooltipKey)}</span>
      </TooltipContent>
      <TooltipTrigger style={{ cursor: "default" }}>
        <span style={{width: "100%", display: "flex", flexDirection: "row"}}>{t(i18nFieldId + ".label")}</span>
      </TooltipTrigger>
    </Tooltip>
    <div style={{paddingTop: "0.5rem"}}>
      {children}
    </div>
  </Label>
}