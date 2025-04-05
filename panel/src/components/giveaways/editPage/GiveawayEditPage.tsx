import "./GiveawayEditPage.css"
import {useCallback} from "react";
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


interface OpenCloseBtnProps {
  status: GiveawayStatus,
  autoStart: Date | undefined,
  onClick: () => void
}

function OpenCloseBtn({status, autoStart, onClick}: OpenCloseBtnProps) {
  let text = "";
  let isEnabled = true;

  switch (status) {
    case GiveawayStatus.CREATE:
      text = "Open NOW";
      break;
    case GiveawayStatus.PAUSED:
      if (autoStart != undefined) {
        text = "Open NOW";
      } else {
        text = "Open";
      }
      break;
    case GiveawayStatus.RUNNING:
      if (autoStart == undefined) {
        text = "Close";
      } else {
        text = "Close NOW";
      }
      break;
    case GiveawayStatus.ARCHIVED:
      isEnabled = false;
      break;
  }
  if (!isEnabled) {
    return ""
  }
  return <Button variant="default" onClick={onClick}>{text}</Button>;
}

export interface GiveawayEditPageProps {
  initialData: Giveaway,
}

export default function GiveawayEditPage({initialData: gw}: GiveawayEditPageProps) {
  const {register, watch, setValue, handleSubmit} = useForm<GiveawaySave>({
    defaultValues: {
      commandPattern: gw.commandPattern,
      allowUserRedraw: gw.allowUserRedraw ? gw.allowUserRedraw : false,
      announceWinnerInChat: gw.announceWinnerInChat ? gw.announceWinnerInChat : false,
      autoCloseTime: gw.autoCloseTime,
      autoStartTime: gw.autoStartTime,
      title: gw.title,
      maxTickets: gw.maxTickets,
      notes: gw.notes,
      ticketCost: gw.ticketCost,
    }
  });

  const refresh = useCallback(() => {
    //TODO check if we are not in create, so that we can actually refresh
  })

  const onSave = useCallback((gw: GiveawaySave) => {
    //TODO confirmation toast when:
    // - change ticket cost
    // - decrement max ticket ammount
    // - change redraw of same user, after first draw
    //TODO if autostart/close time is in past, or same minute, then set it to null
    // (in the request, and in the ui!)
    console.log("SAVING GW");
    console.log(gw)
    //TODO try to refresh data now
  }, []);


  const onArchive = useCallback(() => {
    //TODO find if to archive or unarchive, and do that
  });

  const onOpenClose = useCallback(() => {
    //TODO find if to open or close, and do that
  });

  const onRefund = useCallback(() => {
    //TODO check if refund is currently possible/allowed, and request refund
  })

  const onDraw = useCallback(() => {
    //TODO check if is currently possible to draw, and do that
  })

  return <div className="giveawayEditPage">
    <GwTitleBar onSave={handleSubmit(onSave)} title={gw.title} id={gw.id} lastUpdatedAt={gw.lastUpdatedAt}
                createdAt={gw.createdAt}/>
    <ScrollArea className="contentBorder">
      <div className="formContent">
        <div className="column">
          <InputVL i18nFieldId="giveaway.edit.title" {...register("title")}/>
          <TextareaVL i18nFieldId="giveaway.edit.notes" {...register("notes")}/>
          <InputVL i18nFieldId="giveaway.edit.commandPattern" {...register("commandPattern")}/>
          <InputVL i18nFieldId="giveaway.edit.startTime" type="time" {...register("autoStartTime")}/>
          <InputVL i18nFieldId="giveaway.edit.endTime" type="time" {...register("autoCloseTime")}/>
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
            <OpenCloseBtn onClick={onOpenClose} autoStart={gw.autoStartTime} status={gw.status}/>
            {gw.status == GiveawayStatus.RUNNING || gw.status == GiveawayStatus.PAUSED ? <>
              <Button onClick={onDraw} variant="default">Draw</Button>
              <Button onClick={onRefund} variant="destructive">Refund All Tickets</Button>
              {/*TODO show but disable these buttons if there are no tickets bought */}
            </> : ""}
            {gw.status != GiveawayStatus.CREATE ?
              <Button onClick={onArchive}
                      variant="default">{gw.status == GiveawayStatus.ARCHIVED ? "Unarchive" : "Archive"}</Button>
              : ""}
            <Button onClick={handleSubmit(onSave)} variant="default">Save</Button>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
}