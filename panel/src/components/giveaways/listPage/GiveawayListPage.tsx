import "./GiveawayListPage.css"
import Placeholder from "@c/Placeholder/Placeholder.tsx";
import GiveawayTemplateList from "./GiveawayTemplateList.tsx";
import GiveawayList from "./GiveawayList.tsx";
import {useState} from "react";
import GiveawayEditPage from "@c/giveaways/editPage/GiveawayEditPage.tsx";
import {Giveaway} from "@c/giveaways/Giveaway.ts";

export default function GiveawayListPage() {
  const [createGw, setCreateGw] = useState<Giveaway | undefined>(undefined)
  if (createGw) {
    return <GiveawayEditPage initialData={createGw}/>
  }
  return <div className="giveawayListView">
    <GiveawayList/>
    <div className="columnRight">
      <GiveawayTemplateList handleCreateGw={setCreateGw}/>
      <Placeholder/>
    </div>
  </div>
}