import AuthTile from "./AuthTile.tsx";
import "./AuthList.css"
import {Loader} from "lucide-react";
import useData from "../../common/useData.ts";

export interface Oauth {
  service: string,
  accName: string,
  url: string,
}

export default function OauthSetup() {
  const {loading, data} = useData<Oauth[]>("/setup/auth/list", "Oauth Status", [])

  if (loading)
    return <Loader/>

  if (data == undefined || data.length == 0) {
    return <div className="authNotNeeded"><h1>All accounts are setup!</h1></div>
  }

  return <div className="authListContainer">
    <h1>External accounts that need to be setup:</h1>
    <div className="authList">
      {data.map((oauth, index) => <AuthTile key={index} oauth={oauth}/>)}
    </div>
  </div>
}