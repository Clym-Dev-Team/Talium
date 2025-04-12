import {PANEL_BASE_URL} from "@/main.tsx";

export default function TokenRemover() {
  const query = new URLSearchParams(document.location.hash.substring(1))
  const state = query.get("state")
  const token = query.get("access_token")
  const scope = query.get("scope")
  const tokenType = query.get("token_type")
  const error = query.get("error")
  const error_description = query.get("error_description")

  if (error == null && token != null) {
    //TODO check state
    localStorage.setItem("accessToken", token)
    setTimeout(() => location.assign(PANEL_BASE_URL), 1)
  }
  //TODO error handling

  return <div>
    Login {error == null ? "successfull" : "unsuccessfull"} <br/>
    state: {state} <br/>
    token: {token} <br/>
    scope: {scope} <br/>
    tokenType: {tokenType} <br/>
    error: {error} <br/>
    error_description: {error_description} <br/>
  </div>
}