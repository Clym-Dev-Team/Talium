import "./watchtime.css"
import Placeholder from "@c/Placeholder/Placeholder.tsx";
import WatchtimeLeaderboard from "./WatchtimeLeaderboard.tsx";
import WatchtimeEditor from "./WatchtimeEditor.tsx";

export default function WatchtimePage() {

  return <div className="watchtimePage">
    <div className="column">
      <WatchtimeLeaderboard/>
    </div>
    <div className="column">
      <WatchtimeEditor/>
      <Placeholder bottomText="maybe coin settings and bulk edit options in accordions"/>
    </div>
  </div>
}