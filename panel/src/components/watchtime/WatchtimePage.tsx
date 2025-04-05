import "./watchtime.css"
import WatchtimeLeaderboard from "./WatchtimeLeaderboard.tsx";
import WatchtimeEditor from "./WatchtimeEditor.tsx";
import Placeholder from "@c/Placeholder/Placeholder.tsx";

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