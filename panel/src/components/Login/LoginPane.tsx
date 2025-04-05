import "./LoginPane.css"
import {Link} from "react-router-dom";
import {PANEL_BASE_URL, TWITCH_CLIENT_ID} from "@/main.tsx";

export default function LoginPane() {
  const redirectUri = PANEL_BASE_URL + "/twitchToken";
  const scopes = ""
  const state = window.crypto.randomUUID();

  const link = new URL("https://id.twitch.tv/oauth2/authorize")
  link.searchParams.append("response_type", "token")
  link.searchParams.append("client_id", TWITCH_CLIENT_ID)
  link.searchParams.append("redirect_uri", redirectUri)
  link.searchParams.append("scope", scopes)
  link.searchParams.append("state", state)

  window.location.href = link.href;

  return <div className="login">
    <Link to={link.href}>Login with Twitch</Link>
  </div>
}