import "./GiveawayTemplateList.css"
import {Button} from "../../../../@shadcn/components/ui/button.tsx";
import IconGear from "../../../assets/IconGear.tsx";
import useData from "../../../common/useData.ts";
import Loader from "../../../common/LoadingSpinner/Loader.tsx";
import {useCallback} from "react";

export interface GiveawayTemplateListProps {
}

interface TemplateItem {
  displayName: string;
  id: string,
}

export default function GiveawayTemplateList({}: GiveawayTemplateListProps) {
  const {loading, data, sendData} = useData<TemplateItem[]>("/giveaway/templates", "Giveaway Templates", [])
  const templateList: TemplateItem[] = [
    {
      displayName: "test Template 1",
      id: "test1"
    },
    {
      displayName: "Goldkette",
      id: "goldkette"
    },
    {
      displayName: "Steam Key",
      id: "steamKey"
    }
  ]

  const onCreate = useCallback((id: string) => {
    sendData("/giveaway/fromTemplate/" + encodeURIComponent(id), "Successfully created Giveaway from template", {method: "POST"})
      .then(res => {
        console.log("giveaway ID")
        console.log(res)
      })
    //TODO redirect to gw editor page for this
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