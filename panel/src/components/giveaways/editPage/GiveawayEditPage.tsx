import "./GiveawayEditPage.css"
import {Button} from "@shadcn/button.tsx";
import VLabel from "@/common/VLabel.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "@shadcn/select.tsx";
import {useForm} from "react-hook-form";
import CheckBar from "@/common/BeanBox/CheckBar.tsx";
import ComingSoon from "@/common/CommingSoon/ComingSoon.tsx";
import WinnerCard from "./ticketCards/WinnerCard.tsx";
import TicketCard from "./ticketCards/TicketCard.tsx";
import {useCallback} from "react";
import InputVL from "@/common/InputVL.tsx";
import TextareaVL from "@/common/TextAreaVL.tsx";
import IconSave from "@/assets/IconSave.tsx";
import InfoData from "@/common/InfoData.tsx";
import {Giveaway} from "@/components/giveaways/Giveaway.ts";
import {GiveawaySave} from "@/components/giveaways/GiveawaySave.ts";
import {ScrollArea} from "@/components/ui/scroll-area.tsx";
import TemplateEditor from "@/components/Commands/common/templates/TemplateEditor.tsx";

export default function GiveawayEditPage() {
  const data: Giveaway = {}
  // const {data, loading, sendData} = useData<Giveaway | undefined>("/giveawas/", "Giveaway", undefined);
  const {register, watch, setValue, handleSubmit} = useForm<GiveawaySave>({
    defaultValues: {
      commandPattern: data.commandPattern,
      allowUserRedraw: data.allowUserRedraw ? data.allowUserRedraw : false,
      announceWinnerInChat: data.announceWinnerInChat ? data.announceWinnerInChat : false,
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
      <Button variant="default" className="saveBtn" onClick={handleSubmit(submit)}><IconSave/></Button>
      <span>Edit: {data.title}</span>
      <div className="infoBoxes">
        <InfoData i18nFieldId={"giveaway.edit.gwId"} data={data.id}/>
        <InfoData i18nFieldId={"giveaway.edit.createdAt"} data={data.createdAt}/>
        <InfoData i18nFieldId={"giveaway.edit.lastUpdatedAt"} data={data.lastUpdatedAt}/>
      </div>
    </div>
    <ScrollArea className="contentBorder">
      <div className="formContent">
        <div className="column">
          <InputVL i18nFieldId="giveaway.edit.title" {...register("title")}/>
          <TextareaVL i18nFieldId="giveaway.edit.notes" {...register("notes")}/>
          <InputVL i18nFieldId="giveaway.edit.commandPattern" {...register("commandPattern")}/>
          <InputVL i18nFieldId="giveaway.edit.startTime" type="time" {...register("startTime")}/>
          <InputVL i18nFieldId="giveaway.edit.endTime" type="time" {...register("endTime")}/>
          <InputVL i18nFieldId="giveaway.edit.ticketCost" type="number" {...register("ticketCost")}/>
          <InputVL i18nFieldId="giveaway.edit.maxTickets" type="number" {...register("maxTickets")}/>
          <CheckBar checked={watch("allowUserRedraw")} onChange={b => setValue("allowUserRedraw", b)}>
            Allow Redraw of User
          </CheckBar>
          <CheckBar checked={watch("announceWinnerInChat")} onChange={b => setValue("announceWinnerInChat", b)}>
            Announce Winner in Chat
          </CheckBar>
          {/*<GWPolicySelector/>*/}
        </div>

        {/* ------- COLUMN 2 ------- */}

        <div className="column">
          <ComingSoon>
            <h1>Reminder Timer</h1>
            <CheckBar checked={false} onChange={_ => {
            }}>
              Enable Reminder Message Timer
            </CheckBar>
            <VLabel i18nFieldId="giveaway.edit.policy"><Select>
              <SelectTrigger>Select the Timer Group to add the Message to</SelectTrigger>
              <SelectContent className="dark">
                <SelectItem value="GW-TIMER">Gewinnspiel</SelectItem>
                <SelectItem value="SOME-TIMER1">Live-Erinnerung</SelectItem>
                <SelectItem value="SOME-TIMER2">Jeweils alle 25 Minuten</SelectItem>
                <SelectItem value="SOME-TIMER2">Booster (alle 30 Minuten)</SelectItem>
              </SelectContent>
            </Select></VLabel>
            <VLabel i18nFieldId="giveaway.edit.timerTemplate">
              <TemplateEditor register={undefined} varSchema=""/>
            </VLabel>
            {/* TODO add template color field*/}
            <h1>Public Website</h1>
            <InputVL i18nFieldId="giveaway.edit.imageUrl" type="url"/>
            <TextareaVL i18nFieldId="giveaway.edit.publicDescription"/>
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