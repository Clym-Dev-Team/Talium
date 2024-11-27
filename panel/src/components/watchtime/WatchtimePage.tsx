import "./watchtime.css"
import WatchtitmeLeaderboard from "./WatchtitmeLeaderboard.tsx";
import Placeholder from "../Placeholder/Placeholder.tsx";
import WatchtimeEditor from "./WatchtimeEditor.tsx";
export default function WatchtimePage() {

  return <div className="watchtimePage">
    <div className="column">
      <WatchtitmeLeaderboard/>
    </div>
    <div className="column">
      <WatchtimeEditor/>
      <Placeholder bottomText="maybe coin settings and bulk edit options in accordions"/>
    </div>
  </div>
}