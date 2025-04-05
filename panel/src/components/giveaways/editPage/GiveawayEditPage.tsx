import "./GiveawayEditPage.css"
import {useCallback, useEffect, useState} from "react";
import {useForm} from "react-hook-form";
import {Select, SelectContent, SelectItem, SelectTrigger} from "@shadcn/select.tsx";
import {Button} from "@shadcn/button.tsx";
import VLabel from "@s/VLabel.tsx";
import TextareaVL from "@s/TextAreaVL.tsx";
import InputVL from "@s/InputVL.tsx";
import ComingSoon from "@s/comingSoon/ComingSoon.tsx";
import CheckBar from "@s/checkBar/CheckBar.tsx";
import {ScrollArea} from "@c/ui/scroll-area.tsx";
import {GiveawaySave} from "@c/giveaways/GiveawaySave.ts";
import {Giveaway, GiveawayStatus} from "@c/giveaways/Giveaway.ts";
import TemplateEditor from "@c/Commands/common/templates/TemplateEditor.tsx";
import {TicketResultsPanel} from "@c/giveaways/editPage/TicketResultsPanel.tsx";
import {GwAuditLogs} from "@c/giveaways/editPage/GwAuditLogs.tsx";
import {GwTitleBar} from "@c/giveaways/editPage/GwTitleBar.tsx";

export interface GiveawayEditPageProps {
  isCreate: false
}

interface OpenCloseBtnProps {
  status: GiveawayStatus,
  autoStart: Date | undefined,
}

function OpenCloseBtn({status, autoStart}: OpenCloseBtnProps) {
  const [text, setText] = useState("")
  const [isEnabled, setIsEnabled] = useState(true)

  useEffect(() => {
      switch (status) {
        case GiveawayStatus.CREATED:
          setText("Open NOW");
          break;
        case GiveawayStatus.PAUSED:
          if (autoStart != undefined) {
            setText("Open NOW");
          } else {
            setText ("Open");
          }
          break;
        case GiveawayStatus.RUNNING:
          if (autoStart != undefined) {
            setText("Close NOW");
          } else {
            setText("Close");
          }
          break;
        case GiveawayStatus.ARCHIVED:
          setIsEnabled(false);
          break;
      }
  }, [status, autoStart]);
  if (!isEnabled) {
    return ""
  }
  return <Button variant="default">{text}</Button>;
}

export default function GiveawayEditPage({isCreate}: GiveawayEditPageProps) {
  const data: Giveaway = {
    status: GiveawayStatus.CREATED,
  }
  // const {data, loading, sendData} = useData<Giveaway | undefined>("/giveawas/", "Giveaway", undefined);
  const {register, watch, setValue, handleSubmit} = useForm<GiveawaySave>({
    defaultValues: {
      commandPattern: data.commandPattern,
      allowUserRedraw: data.allowUserRedraw ? data.allowUserRedraw : false,
      announceWinnerInChat: data.announceWinnerInChat ? data.announceWinnerInChat : false,
      autoCloseTime: data.autoCloseTime,
      autoStartTime: data.autoStartTime,
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
    <GwTitleBar onSave={handleSubmit(submit)} title={data.title} id={data.id} lastUpdatedAt={data.lastUpdatedAt}
                createdAt={data.createdAt}/>
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
          {/*<GwPolicySelector/>*/}
        </div>

        {/* ------- COLUMN 2 ------- */}

        <div className="column">
          <ComingSoon>
            <h1>Reminder Timer</h1>
            <CheckBar checked={false} onChange={_ => {
            }}>Enable Reminder Message Timer</CheckBar>
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
          <TicketResultsPanel/>
          <ComingSoon>
            <GwAuditLogs/>
          </ComingSoon>
          <div className="dangerArea">
            <OpenCloseBtn autoStart={data.autoStartTime} status={data.status}/>
            <Button variant="destructive">Refund All Tickets</Button>
            <Button variant="default" onClick={handleSubmit(submit)}>Save & Exit</Button>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
}