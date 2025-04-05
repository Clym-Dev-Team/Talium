import {useCallback} from "react";
import {Button} from "@shadcn/button.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import IconGear from "@i/IconGear.tsx";
import "./GiveawayTemplateList.css"

interface TemplateItem {
  displayName: string;
  id: string,
}

export default function GiveawayTemplateList() {
  const {loading, data, sendData} = useData<TemplateItem[]>("/giveaway/templates", "Giveaway Templates", [])

  const onCreate = useCallback((id: string) => {
    sendData("/giveaway/fromTemplate/" + encodeURIComponent(id), "Successfully created Giveaway from template", {method: "POST"})
      .then(res => {
        window.location.href = "/giveawayEdit/" + res;
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