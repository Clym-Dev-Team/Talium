import "./GiveawayTemplateList.css"
import {useCallback} from "react";
import {Button} from "@shadcn/button.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/LoadingSpinner/Loader.tsx";
import IconGear from "@i/IconGear.tsx";

interface TemplateItem {
  displayName: string;
  id: string,
}

export default function GiveawayTemplateList() {
  const {loading, data, sendData} = useData<TemplateItem[]>("/giveaway/templates", "Giveaway Templates", [])

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