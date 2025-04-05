import {useState} from "react";
import {useForm} from "react-hook-form";
import {useToast} from "@shadcn/use-toast.ts";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import {fetchWithAuth} from "@c/Login/LoginPage.tsx";
import VLabel from "@s/VLabel.tsx";
import IconSave from "@i/IconSave.tsx";
import IconCloudDown from "@i/IconCloudDown.tsx";

interface ChatterDTO {
  userId: string,
  coins: number,
  watchtimeSeconds: number,
}

export default function WatchtimeEditor() {
  const {toast} = useToast();
  const [loadedUser, setLoadedUser] = useState(false);
  const [userName, setUserName] = useState<string>()
  const {handleSubmit, register, reset} = useForm<ChatterDTO>({disabled: !loadedUser});

  function loadData() {
    fetchWithAuth("/watchtime/username/" + userName)
      .then(response => response.json())
      .then(value => reset(value))
      .then(() => setLoadedUser(true))
      .catch(reason => {
        if (reason == "Error: 404 ") {
          toast({
            className: "toast toast-failure",
            title: "User not found",
            description: "User: " + userName,
          })
          return
        }
        toast({
          className: "toast toast-failure",
          title: "ERROR loading User",
          description: reason.toString()
        });
      })
  }

  function submit(chatter: ChatterDTO) {
    fetchWithAuth("/watchtime/update", {method: "POST", body: JSON.stringify(chatter)})
      .then(() => toast({className: "toast toast-success", title: "Saved Watchtime!"}))
      .then(() => setLoadedUser(false))
      .catch(reason => toast({
        className: "toast toast-failure",
        title: "ERROR loading User",
        description: reason.toString()
      }))
  }

  return <div className="editTile">
    <h1>Edit Watchtime and Coins Data:</h1>
    <form className="searchBox" onSubmit={event => {
      event.preventDefault()
    }}>
      <Input placeholder="Search for twitch username..." onChange={event => setUserName(event.target.value)}/>
      <Button type="submit" onClick={loadData}><IconCloudDown/></Button>
    </form>
    <div className="dataBox">
      <VLabel i18nFieldId="Watchtime [seconds]">
        <Input placeholder="..." {...register("watchtimeSeconds")}/>
      </VLabel>
      <VLabel i18nFieldId="Coins">
        <Input placeholder="..." {...register("coins")}/>
      </VLabel>
    </div>
    <Button onClick={handleSubmit(submit)} disabled={!loadedUser} type="submit">
      <IconSave/>
    </Button>
  </div>

}