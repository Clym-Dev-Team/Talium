import TwitchMenuTile from "./TwitchMenuTile.tsx";
import "./TwitchNavMenu.css"
import IconTimer from "@/assets/IconTimer.tsx";
import IconMegaphone from "@/assets/IconMegaphone.tsx";
import IconTextBox from "@/assets/IconTextBod.tsx";
import IconGift from "@/assets/IconGift.tsx";
import NavLink from "./NavLink.tsx";

export default function TwitchNavMenu() {
  return <div className="navMenu">
    <div className="twitchTiles">
      <TwitchMenuTile allowChildren={false} icon={<IconTextBox/>} label="Commands" target="/commands"/>
      <TwitchMenuTile allowChildren={false} icon={<IconGift/>} label="Giveaways" target="/giveaways"/>
      <TwitchMenuTile allowChildren={false} icon={<IconTimer/>} label="Timer" target="/"/>
      <TwitchMenuTile allowChildren={false} icon={<IconMegaphone/>} label="Alerts" target="/"/>
    </div>
    <div className="otherOptions">
      <NavLink target="/history" name="Message History Test"/>
      <NavLink target="/watchtime" name="Watchtime & Coins"/>
      <NavLink target="/health" name="Health"/>
      <NavLink target="/oauth" name="External accounts"/>
      <NavLink target="/accounts" name="Panel Accounts"/>
    </div>
    <div className="account">
      {/*
      - username
      - your twitch profile picture
      (not sure if we will sink the panel accs to twitch acc's, but we wouldn't need that for this. We could also just have a twitch name field in the account creation panel, and the get the picture for that account
      */}
    </div>
  </div>
}