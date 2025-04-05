import "./OauthResult.tsx"
import {Link} from "react-router-dom";
import {Oauth} from "./OauthSetup.tsx";
import "./AuthTile.css"

export interface AuthTileProps {
  oauth: Oauth
}

export default function AuthTile({oauth}: AuthTileProps) {
  return <Link to={oauth.url}>
    <div className={"authTile"}>
      <span className="serviceName">{oauth.service}</span>
      <span className="accountName">{oauth.accName}</span>
    </div>
  </Link>
}