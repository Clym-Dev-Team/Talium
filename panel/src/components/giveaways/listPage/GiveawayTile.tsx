import {Link} from "react-router-dom";
import {GiveawayPreview} from "@c/giveaways/GiveawayPreview.ts";
import "../GiveawayStatusColors.css"

export interface GiveawayTileProps {
  preview: GiveawayPreview
}

export default function GiveawayTile({preview}: GiveawayTileProps) {
  return <Link to={"/giveawayEdit/" + encodeURIComponent(preview.id)} ><div className={"tile status-" + preview.status}>
    <div className="firstRow">
      <span className="title">{preview.title}</span>
      <span className="status">{preview.status}</span>
    </div>
    <span className="description">{preview.notes}</span>
  </div></Link>;
}