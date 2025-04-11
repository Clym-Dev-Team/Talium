import {useCallback} from "react";
import {Button} from "@shadcn/button.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import IconGear from "@i/IconGear.tsx";
import "./GiveawayTemplateList.css"
import {Giveaway, GiveawayJSON, GiveawayStatus, parseJsonToGiveaway} from "@c/giveaways/Giveaway.ts";

export interface GiveawayTemplateListProps {
  handleCreateGw: (gw: Giveaway) => void;
}

interface TemplateItem {
  displayName: string;
  id: string,
}

export default function GiveawayTemplateList({handleCreateGw}: GiveawayTemplateListProps) {
  const {loading, data, sendData} = useData<TemplateItem[]>("/giveaway/templates", "Giveaway Templates", [])

  const onCreate = useCallback((id: string) => {
    sendData("/giveaway/fromTemplate/" + encodeURIComponent(id), undefined, {method: "POST"})
      .then(res => {
        const gw = parseJsonToGiveaway(res as GiveawayJSON);
        // hardcode CREATED, BE doesn't know this state, so it just sets PAUSED
        gw.status = GiveawayStatus.CREATE;
        handleCreateGw(gw);
      })
  }, [])

  if (loading) {
    return <Loader/>
  }

  return <div className="giveawayTemplateList">
    <div>Giveaway Templates:</div>
    {data.map(template => <Button className="templateItem" key={template.id} variant="secondary" onClick={() => {onCreate(template.id)}}>
      <div>{template.displayName}</div>
      <Button className="templateEditBtn" variant="outline"><IconGear/></Button>
    </Button>)}
  </div>
}