import "./GiveawayEditPage.css"
import {Button} from "../../../../@shadcn/components/ui/button.tsx";
import {ScrollArea} from "../../ui/scroll-area.tsx";
import {Label} from "../../../../@shadcn/components/ui/label.tsx";
import VLabel from "../../../common/VerticalLabel/VLabel.tsx";
import {Input} from "../../../../@shadcn/components/ui/input.tsx";
import {Textarea} from "../../../../@shadcn/components/ui/textarea.tsx";
import {Switch} from "../../../../@shadcn/components/ui/switch.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "../../../../@shadcn/components/ui/select.tsx";
import TemplateEditor from "../../Commands/common/templates/TemplateEditor.tsx";
import IconChecked from "../../../assets/IconChecked.tsx";
import IconX from "../../../assets/IconX.tsx";
import {useForm} from "react-hook-form";
import {Command} from "../../Commands/commands/Command.ts";
import BeanCheckBox from "../../../common/BeanBox/BeanCheckBox.tsx";
import {useState} from "react";

export default function GiveawayEditPage() {
  //work arround so that templateEditor is happy
  const [bool1, setBool1] = useState(true);
  const [bool2, setBool2] = useState(false)
  const {register} = useForm<Command>()
  return <div className="giveawayEditPage">
    <div className="tileBar">
      <Button variant="default">Save</Button>
      Edit: !testGW
    </div>
    <ScrollArea className="contentBorder">
      <div className="formContent">
        <div className="column">
          <h1>Giveaway</h1>
          <VLabel name="Giveaway ID"><Input disabled={true}/></VLabel>
          <VLabel name="Giveaway Name"><Input/></VLabel>
          <VLabel name="Notes/Internal Description"><Textarea/></VLabel>
          <VLabel name="Command Pattern"><Input/></VLabel>
          <Textarea/>
          <VLabel name="Autostart Time"><Input type="time"/></VLabel>
          <VLabel name="Autoclose Time"><Input type="time"/></VLabel>
          <VLabel name="Ticket Cost"><Input type="number"/></VLabel>
          <VLabel name="Max Tickets per User"><Input type="number"/></VLabel>
          <BeanCheckBox checked={bool1} onChange={b => setBool1(b)}>Allow Redraw of User</BeanCheckBox>
          <BeanCheckBox checked={bool2} onChange={b => setBool2(b)}>Announce Winner in Chat</BeanCheckBox>
          <VLabel name="Giveaway Policy"><Select>
            <SelectTrigger>Select a Giveaway Policy</SelectTrigger>
            <SelectContent className="dark">
              <SelectItem value="DE-BRIEF">DE-BRIEF</SelectItem>
              <SelectItem value="DE-PACKET">DE-PACKET</SelectItem>
              <SelectItem value="EU-BRIEF">EU-BRIEF</SelectItem>
              <SelectItem value="EU-PACKET">EU-PACKET</SelectItem>
              <SelectItem value="GL-BRIEF">GL-BRIEF</SelectItem>
              <SelectItem value="GL-PACKET">GL-PACKET</SelectItem>
              <SelectItem value="NO-WAITING">NO-WAITING</SelectItem>
            </SelectContent>
          </Select></VLabel>
        </div>
        <div className="column">
          <h1>Reminder Timer</h1>
          <Switch id="timerOn"/>
          <Label htmlFor="timerOn">Enable Reminder Message Timer</Label>
          <VLabel name="Giveaway Policy"><Select>
            <SelectTrigger>Select the Timer Group to add the Message to</SelectTrigger>
            <SelectContent className="dark">
              <SelectItem value="GW-TIMER">Gewinnspiel</SelectItem>
              <SelectItem value="SOME-TIMER1">Live-Erinnerung</SelectItem>
              <SelectItem value="SOME-TIMER2">Jeweils alle 25 Minuten</SelectItem>
              <SelectItem value="SOME-TIMER2">Booster (alle 30 Minuten)</SelectItem>
            </SelectContent>
          </Select></VLabel>
          <VLabel name="Timer Template">
            <TemplateEditor register={register("template.template")} varSchema=""/>
          </VLabel>
          {/* TODO add template color field*/}
          <h1>Public Website</h1>
          <VLabel name="Image Url"><Input type="url"/></VLabel>
          <VLabel name="Public Description"><Textarea/></VLabel>
        </div>
        <div className="column">
          <div className="winnersListCard">
            <h1>Winner List</h1>
            <ScrollArea className="winnersListScrollArea">
              <div className="winnersList">
                <div className="potentialWinner">
                  <div className="username">h28hadi982</div>
                  <div className="potentialWinnerActionBtns">
                    <Button><IconChecked/></Button>
                    <Button><IconX/></Button> {/* nicht da, offene tür als icon oder so, in rot*/}
                    <Button>SWORD</Button> {/* als moderationsgründen abgelehnt, in rot*/}
                  </div>
                </div>
                <hr/>
                <div className="winner">USERNAME dwahdakud</div>
                <div className="winner">dahjkdajkd h</div>
                <div className="winner">u28dha</div>
                <div className="winner">h28hda</div>
              </div>
            </ScrollArea>
          </div>
          <div className="logs">
            <h1>Logs</h1>
          </div>
          <div className="dangerArea">
            <Button variant="default">Start NOW</Button> {/* This button is Start/Pause GW as necessary */}
            <Button variant="destructive">Refund All Tickets</Button>
            <Button variant="default">Save & Exit</Button>
          </div>
        </div>
        {/*<Placeholder/>*/}
      </div>
    </ScrollArea>
  </div>
}