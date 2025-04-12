import {useState} from "react";
import {DeleteIcon, SaveIcon} from "lucide-react";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import {ScrollArea} from "@c/ui/scroll-area.tsx";

interface Account {
  username: string,
  userId: string,
  accountCreationTime: Date
}

export default function AccountsPage() {
  const {data, loading, sendData} = useData("/panelAccounts", "Panel Account", [])
  const [newUsername, setNewUsername] = useState<string>("")

  function onDelete(userId: string, username: string) {
    sendData("/panelAccounts", "Deleted " + username + " Successfully", {method: "DELETE", body: userId})
  }

  function onSubmit() {
    sendData("/panelAccounts", "Created " + newUsername + " Successfully", {method: "POST", body: newUsername})
  }

  if (loading)
    return <Loader/>

  return <div>
    <form onSubmit={() => onSubmit()} style={{ display: "flex" }}>
      <Button><SaveIcon/></Button>
      <Input value={newUsername} onChange={event => setNewUsername(event.target.value)} placeholder="Twitch User(name) to add" />
    </form>
    <ScrollArea>
      <div style={{ display: "flex", width: "10%", justifyContent: "space-between", fontWeight: "bolder" }}>
        <div>Twitch User Name</div>
        <div>Revoke access</div>
      </div>
      {data.map((account: Account) =>
        <div key={account.userId} style={{ display: "flex", width: "10%", justifyContent: "space-between"}}>
          <div>{account.username}</div>
          <Button onClick={() => onDelete(account.userId, account.username)}><DeleteIcon/></Button>
        </div>
      )}
    </ScrollArea>
  </div>
}