import "@c/Commands/common/ListView.css"
import {useState} from "react";
import {Search} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@shadcn/table.tsx";
import {Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle} from "@shadcn/sheet.tsx";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import IsVisibleCheckBox from "@c/Commands/common/IsVisibleCheckbox.tsx";
import EnabledCheckBox from "@c/Commands/common/EnabledCheckBox.tsx";
import {CommandForm} from "./CommandEditSheet.tsx";
import {Command, CommandPermission, CooldownTypes} from "./Command.ts";

const emptyCommand: Command = {
  id: "",
  description: "",
  permission: CommandPermission[CommandPermission.EVERYONE],
  userCooldown: {amount: 0, type: CooldownTypes[CooldownTypes.MESSAGES]},
  globalCooldown: {amount: 0, type: CooldownTypes[CooldownTypes.MESSAGES]},
  isAutoGenerated: false,
  patterns: [{pattern: "", isEnabled: true, isVisible: false, isRegex: false}],
  template: {id: "", template: ""},
  varJsonSchema: ""
}

export default function UserCommandList() {
  const [searchBox, setSearchBox] = useState<string>("")
  const {
    data,
    loading,
    sendData
  } = useData<Command[]>("/commands/userAll?search=" + encodeURIComponent(searchBox), "User-Commands", [])
  const [openCommand, setOpenCommand] = useState<Command | undefined>(undefined)
  const [isNew, setIsNew] = useState(false)

  function handleSearch() {
  }

  if (loading) {
    return <Loader/>
  }

  if (data == undefined) {
    return "ERROR" //TODO
  }

  return <div className="commandList">
    <div className="actionBar">
      <Button className="addCommand" onClick={() => {
        setIsNew(true);
        setOpenCommand(emptyCommand)
      }}>Create a new Command</Button>
      <div className="searchBox">
        <Input placeholder="Search for a Command" value={searchBox}
               onChange={event => setSearchBox(event.target.value)}/>
        <Button onClick={() => handleSearch()}><Search/></Button>
      </div>
    </div>
    <Table>
      <TableHeader>
        <TableHead className="tw-w-16">Enabled</TableHead>
        <TableHead className="tw-w-16">Visible</TableHead>
        <TableHead>Name/Id</TableHead>
        <TableHead>First Pattern</TableHead>
        <TableHead>Template</TableHead>
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
            <TableCell className="tw-w-96">{command.id}</TableCell>
            <TableCell className="tw-w-96">{command.patterns[0].pattern}</TableCell>
            <TableCell>{command.template ? command.template.template : "NO TEMPLATE"}</TableCell>
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