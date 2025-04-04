import "./GiveawayEditPage.css"
import {Button} from "../../../../@shadcn/components/ui/button.tsx";
import {ScrollArea} from "../../ui/scroll-area.tsx";
import VLabel from "../../../common/VerticalLabel/VLabel.tsx";
import {Input} from "../../../../@shadcn/components/ui/input.tsx";
import {Textarea} from "../../../../@shadcn/components/ui/textarea.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "../../../../@shadcn/components/ui/select.tsx";
import TemplateEditor from "../../Commands/common/templates/TemplateEditor.tsx";
import {useForm} from "react-hook-form";
import BeanCheckBox from "../../../common/BeanBox/BeanCheckBox.tsx";
import ComingSoon from "../../../common/CommingSoon/ComingSoon.tsx";
import WinnerCard from "./ticketCards/WinnerCard.tsx";
import TicketCard from "./ticketCards/TicketCard.tsx";
import useData from "../../../common/useData.ts";
import {Giveaway} from "../Giveaway.ts";
import {GiveawaySave} from "../GiveawaySave.ts";
import {useCallback} from "react";
import InputVL from "../../../common/InputVL.tsx";
import TextareaVL from "../../../common/TextAreaVL.tsx";

export default function GiveawayEditPage() {
  const data: Giveaway = {}
  if (false) {
    const {data, loading, sendData} = useData<Giveaway | undefined>("/giveawas/", "Giveaway", undefined);
  }
  const {register, watch, setValue, handleSubmit} = useForm<GiveawaySave>({
    defaultValues: {
      commandPattern: data.commandPattern,
      allowUserRedraw: data.allowUserRedraw? data.allowUserRedraw: false,
      announceWinnerInChat: data.announceWinnerInChat? data.announceWinnerInChat: false,
      endTime: data.endTime,
      startTime: data.startTime,
      title: data.title,
      maxTickets: data.maxTickets,
      notes: data.notes,
      ticketCost: data.ticketCost,
    }
  });

  const submit = useCallback((gw: GiveawaySave) => {
    //TODO confirmation toast when:
    // - change ticket cost
    // - decrement max ticket ammount
    // - change redraw of same user, after first draw
    //TODO if autostart/close time is in past, or same minute, then set it to null
    // (in the request, and in the ui!)
    console.log("SAVING GW");
    console.log(gw)
  }, []);

  return <div className="giveawayEditPage">
    <div className="tileBar">
      <Button variant="default">Save</Button>
      {/*TODO Style better bigger bolder, and with GW Title */}
      Edit: !testGW
    </div>
    <ScrollArea className="contentBorder">
      <div className="formContent">
        <div className="column">
          <h1>Giveaway</h1>
          <InputVL label="Giveaway ID" disabled={true} value={data.id} hoverText="Internal unique Giveaway Identifier. You wil always be able to uniquely find this Giveaway under this ID, even if other Giveaways have the same title. Although, you will rarely need to use this"/>
          <InputVL label="Giveaway Title" {...register("title")}/>
          <TextareaVL label="Notes/Internal Description" {...register("notes")}/>
          <InputVL label="Command Pattern" {...register("commandPattern")}/>
          <InputVL label="Autostart Time" type="time" {...register("startTime")}/>
          <InputVL label="Autoclose Time" type="time" {...register("endTime")}/>
          <InputVL label="Ticket Cost" type="number" {...register("ticketCost")}/>
          <InputVL label="Max Tickets per User" type="number" {...register("maxTickets")}/>
          <BeanCheckBox checked={watch("allowUserRedraw")} onChange={b => setValue("allowUserRedraw", b)}>
            Allow Redraw of User
          </BeanCheckBox>
          <BeanCheckBox checked={watch("announceWinnerInChat")} onChange={b => setValue("announceWinnerInChat", b)}>
            Announce Winner in Chat
          </BeanCheckBox>
          {/*<GWPolicySelector/>*/}
        </div>

        {/* ------- COLUMN 2 ------- */}

        <div className="column">
          <ComingSoon>
            <h1>Reminder Timer</h1>
            <BeanCheckBox checked={false} onChange={_ => {
            }}>
              Enable Reminder Message Timer
            </BeanCheckBox>
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
              <TemplateEditor register={undefined} varSchema=""/>
            </VLabel>
            {/* TODO add template color field*/}
            <h1>Public Website</h1>
            <InputVL label="Image Url" type="url"/>
            <TextareaVL label="Public Description"/>
          </ComingSoon>
        </div>

        {/* ------- COLUMN 3 ------- */}

        <div className="column">
          {/*TODO Move this entire WinnerListCard into own component*/}
          <div className="winnersListCard">
            <ScrollArea className="winnersListScrollArea">
              <div className="winnerList">
                <div className="sectionTitle">
                  <h3 className="sectionTitleText">Winner:</h3>
                  <span className="legend">
                    <span>Username</span>
                    <span>Actions</span>
                  </span>
                </div>
                <WinnerCard username="testUser82834834"/>
                <WinnerCard username="hdh82"/>
                <WinnerCard username="cookie"/>
                <WinnerCard username="supie"/>
                <WinnerCard username="clym"/>
              </div>
              <hr/>
              <div className="ticketList">
                <div className="sectionTitle">
                  <h3 className="sectionTitleText">Submitted Tickets:</h3>
                  <span className="legend">
                    <span>Username</span>
                    <span>Ticket Amount</span>
                  </span>
                </div>
                <TicketCard tickets={98393} username="zuser23"/>
                <TicketCard tickets={1} username="user8239239"/>
                <TicketCard tickets={9} username="jfk"/>
              </div>
            </ScrollArea>
          </div>
          <ComingSoon>
            <div className="logs">
              <h1 style={{height: "4rem"}}>Logs</h1>
            </div>
          </ComingSoon>
          <div className="dangerArea">
            <Button variant="default">Start NOW</Button> {/* This button is Start/Pause GW as necessary */}
            <Button variant="destructive">Refund All Tickets</Button>
            <Button variant="default" onClick={handleSubmit(submit)}>Save & Exit</Button>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
}