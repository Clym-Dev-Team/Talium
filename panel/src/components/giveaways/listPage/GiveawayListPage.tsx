import "./GiveawayListPage.css"
import Placeholder from "@c/Placeholder/Placeholder.tsx";
import GiveawayTemplateList from "./GiveawayTemplateList.tsx";
import GiveawayList from "./GiveawayList.tsx";

export default function GiveawayListPage() {
  return <div className="giveawayListView">
    <GiveawayList/>
    <div className="columnRight">
      <GiveawayTemplateList/>
      <Placeholder/>
    </div>
  </div>
}