import "./GiveawayListPage.css"
import GiveawayTemplateList from "./GiveawayTemplateList.tsx";
import GiveawayList from "./GiveawayList.tsx";
import Placeholder from "@/components/Placeholder/Placeholder.tsx";

export default function GiveawayListPage() {
  return <div className="giveawayListView">
    <GiveawayList/>
    <div className="columnRight">
      <GiveawayTemplateList/>
      <Placeholder/>
    </div>
  </div>
}