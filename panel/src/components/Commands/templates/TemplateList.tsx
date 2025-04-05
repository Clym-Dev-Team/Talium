import "@c/Commands/common/ListView.css"
import {useState} from "react";
import {Search} from "lucide-react";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@shadcn/table.tsx";
import {Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle} from "@shadcn/sheet.tsx";
import {Input} from "@shadcn/input.tsx";
import {Button} from "@shadcn/button.tsx";
import WarningBox from "@s/warning/WarningBox.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import TemplateForm from "./TemplateForm.tsx";
import {Template} from "./Template.ts";

export default function TemplateList() {
  const [searchBox, setSearchBox] = useState<string>("")
  const {data, loading, sendData} = useData<Template[]>("/template/all?search=" + encodeURIComponent(searchBox), "Templates", [])
  const [openTemplate, setOpenTemplate] = useState<Template | undefined>(undefined)

  function handleSearch() {}

  if (loading) {
    return <Loader/>
  }

  if (data == undefined) {
    return "ERROR" //TODO
  }

  return <div className="commandList">
    <WarningBox>This Page lists all templates/strings/texts that the bot uses to generate output text/messages. Templates are triggered by commands, giveaways, timers, alerts, etc... and are used to construct the messages that are hen send into the twitch chat (or to other platforms like discord) <br/>
      If you delete a template that was registered automatically it will be regenerated with the next startup. But you can void the output of a template by deleting the content of the template.
    </WarningBox>
    <div className="actionBar">
      <div className="searchBox">
        <Input placeholder="Search for a Template" value={searchBox} onChange={event => setSearchBox(event.target.value)}/>
        <Button onClick={() => handleSearch()}><Search/></Button>
      </div>
    </div>
    <Table>
      <TableHeader>
        <TableHead>Name/Id</TableHead>
        <TableHead>Template Text</TableHead>
      </TableHeader>
      <TableBody>
        {data.map((template) => (
          <TableRow key={template.id} onClick={() => {
            setOpenTemplate(template)
          }}>
            <TableCell>{template.id}</TableCell>
            <TableCell>{template.template}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
    <Sheet open={openTemplate != undefined} onOpenChange={open => {
      !open && setOpenTemplate(undefined)
    }}>
      <SheetContent style={{minWidth: "40%", overflowY: "auto"}} className="dark">
        <SheetHeader>
          <SheetTitle>Edit Template:</SheetTitle>
          <SheetDescription>Edit a Template</SheetDescription>
        </SheetHeader>
        {openTemplate ? <TemplateForm template={openTemplate}
                                      onDelete={(id) => sendData("/template/delete/" + id, "Deleted Template successfully!", {method: "DELETE"})}
                                      onSubmit={template => sendData("/template/save", "Template saved successfully!", {
                                      method: "POST",
                                      body: JSON.stringify(template)
                                    })}/> : ""}
      </SheetContent>
    </Sheet>
  </div>
}