import "@/components/Commands/common/ListView.css"
import {CommandForm} from "./CommandEditSheet.tsx";
import {Command} from "./Command.ts";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@shadcn/table.tsx";
import {Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle} from "@shadcn/sheet.tsx";
import {useState} from "react";
import Loader from "@/common/LoadingSpinner/Loader.tsx";
import useData from "@/common/useData.ts";
import {Button} from "@shadcn/button.tsx";
import {Input} from "@shadcn/input.tsx";
import {Search} from "lucide-react";
import WarningBox from "@/common/warning/WarningBox.tsx";
import IsVisibleCheckBox from "@/components/Commands/common/IsVisibleCheckbox.tsx";
import EnabledCheckBox from "@/components/Commands/common/EnabledCheckBox.tsx";

export default function AllCommandList() {
  const [searchBox, setSearchBox] = useState<string>("")
  const {data, loading, sendData} = useData<Command[]>("/commands/all?search=" + encodeURIComponent(searchBox), "Commands", [])
  const [openCommand, setOpenCommand] = useState<Command | undefined>(undefined)
  const [isNew, setIsNew] = useState(false)

  function handleSearch() {}

  if (loading) {
    return <Loader/>
  }

  if (data == undefined) {
    return "ERROR" //TODO
  }

  return <div className="commandList">
    <WarningBox>This Page lists all commands that are registered in the Bot, this includes internal command that are
      used for various other features like giveaways and management features for mods. <br/>
      Most of these Commands don't have a template (text output), instead they call an internal function of the Bot that
      can then process the command action and may then respond with different or multiple templates. <br/>
      If you delete a command that was registered automatically it will be regenerated with the next startup. Instead,
      if you want to disable the command disable all its patterns. You can still use the same patterns in other
      commands.
    </WarningBox>
    <div className="actionBar">
      <div className="searchBox">
        <Input placeholder="Search for a Command" value={searchBox} onChange={event => setSearchBox(event.target.value)}/>
        <Button onClick={() => handleSearch()}><Search/></Button>
      </div>
    </div>
    <Table>
      <TableHeader>
        <TableHead className="tw-w-16">Enabled</TableHead>
        <TableHead className="tw-w-16">Visible</TableHead>
        <TableHead>Name/Id</TableHead>
        <TableHead>First Pattern</TableHead>
      </TableHeader>
      <TableBody>
        {data.map((command) => (
          <TableRow key={command.id} onClick={() => {
            setIsNew(false);
            setOpenCommand(command)
          }}>
            <TableCell><span className="centerInColumn">
              <EnabledCheckBox checked={command.patterns[0].isEnabled}
                               onChange={checked => sendData(`/commands/id/${command.id}/set/enabled`, `Set all Patterns to ${checked ? "enabled" : "disabled"}`, {
                                 method: "POST",
                                 body: `${checked}`
                               })}/>
            </span></TableCell>
            <TableCell><span className="centerInColumn">
              <IsVisibleCheckBox checked={command.patterns[0].isVisible}
                                 onChange={checked => sendData(`/commands/id/${command.id}/set/visible`, `Set all Patterns to ${checked ? "visible" : "hidden"}`, {
                                   method: "POST",
                                   body: `${checked}`
                                 })}/>
            </span></TableCell>
            <TableCell>{command.id}</TableCell>
            <TableCell>{command.patterns[0].pattern}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
    <Sheet open={openCommand != undefined} onOpenChange={open => {
      !open && setOpenCommand(undefined)
    }}>
      <SheetContent style={{minWidth: "40%", overflowY: "auto"}} className="dark">
        <SheetHeader>
          <SheetTitle>Edit Command:</SheetTitle>
          <SheetDescription>Edit a command. All empty trigger will be ignored</SheetDescription>
        </SheetHeader>
        {openCommand ? <CommandForm command={openCommand} isNew={isNew}
                                    onDelete={(id) => sendData("/commands/delete/" + id, "Deleted Command successfully!", {method: "DELETE"})}
                                    onSubmit={command => sendData("/commands/save", "Command saved successfully!", {
                                      method: "POST",
                                      body: JSON.stringify(command)
                                    })}/> : ""}
      </SheetContent>
    </Sheet>
  </div>
}