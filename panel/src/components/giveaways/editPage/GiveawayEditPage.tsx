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
import {usePopout} from "@s/popoutProvider/PopoutProvider.tsx";
import DisruptiveActionPopup from "@c/giveaways/DisruptiveActionPopup.tsx";
import {fetchWithAuth} from "@c/Login/LoginPage.tsx";
import {useToast} from "@shadcn/use-toast.ts";


enum GiveawayAction {
  OPEN,
  CLOSE,
  DRAW,
  REFUNDALL,
  ARCHIVE,
  UNARCHIVE,
  DELETE
}

export interface GiveawayEditPageProps {
  initialData: Giveaway,
}

/**
 * Shows an editor to edit the giveaway in the props.
 * The giveaway data does not need to exist in the DB, if it does not exist, it will be created.
 *
 * After the giveaway is saved, the page is reloaded!
 */
export default function GiveawayEditPage({initialData: gw}: GiveawayEditPageProps) {
  const {toast} = useToast();
  const {showPopout} = usePopout()
  const {register, watch, setValue, handleSubmit, getFieldState} = useForm<GiveawaySave>({
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

  const doSave = useCallback((save: GiveawaySave) => {
    if (gw.status == GiveawayStatus.CREATE) {
      fetchWithAuth("/giveaway/create", {
        method: "POST",
        body: JSON.stringify(save),
      }).then(() => location.reload())
        .catch(reason => toast({
          className: "toast toast-failure",
          title: "ERROR creating Giveaway",
          description: reason.toString()
        }));
    } else {
      fetchWithAuth("/giveaway/save/" + gw.id, {
        method: "POST",
        body: JSON.stringify(save),
      }).then(() => location.reload())
        .catch(reason => toast({
          className: "toast toast-failure",
          title: "ERROR saving giveaway",
          description: reason.toString()
        }));
    }
  }, [])

  const onSave = useCallback((save: GiveawaySave) => {
    if (gw.status == GiveawayStatus.CREATE) {
      doSave(save);
      return
    }

    const warnings: string[] = [];
    if (getFieldState("ticketCost").isDirty) {
      warnings.push("changed Ticket cost");
    }
    if (getFieldState("maxTickets").isDirty && gw.maxTickets > save.maxTickets) {
      warnings.push("decreased Maximum amount of Tickets per person");
    }
    if (getFieldState("allowUserRedraw") && gw.winnerList?.length > 0) {
      warnings.push("changed 'allow redraw of same user' after a user was already drawn")
    }
    if (save.autoStartTime != null && save.autoStartTime <= new Date()) {
      save.autoStartTime = null;
    }
    if (save.autoCloseTime != null && save.autoCloseTime <= new Date()) {
      save.autoCloseTime = null;
    }
    if (warnings.length > 0) {
      showPopout(<DisruptiveActionPopup warnings={warnings} onConfirm={() => {
        doSave(save);
      }}/>)
    } else {
      doSave(save);
    }
  }, []);

  const onAction = useCallback((action: GiveawayAction) => {
    const actionString = (GiveawayAction as any)[action] as GiveawayAction;
    fetchWithAuth("/giveaway/action/" + gw.id + "/" + actionString, {
      method: "POST",
    }).then(() => location.reload())
      .catch(reason => toast({
        className: "toast toast-failure",
        title: "ERROR executing action: " + action,
        description: reason.toString()
      }));
  }, []);

  return <div className="giveawayEditPage">
    <GwTitleBar onSave={handleSubmit(onSave)} title={gw.title} id={gw.id} lastUpdatedAt={gw.lastUpdatedAt}
                createdAt={gw.createdAt} status={gw.status}/>
    <ScrollArea className={"contentBorder status-" + ((GiveawayStatus as any)[gw.status] as GiveawayStatus)}>
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
            <Button variant="default" onClick={() => onAction(gw.status == GiveawayStatus.RUNNING ? GiveawayAction.CLOSE : GiveawayAction.OPEN)}>
              {gw.status == GiveawayStatus.RUNNING ? "Close" : "Open"}
            </Button>
            {(gw.status == GiveawayStatus.RUNNING || gw.status == GiveawayStatus.PAUSED) &&
              <>
                <Button onClick={() => onAction(GiveawayAction.DRAW)} variant="default" disabled={gw.ticketList.length === 0}>Draw</Button>
                <Button onClick={() => onAction(GiveawayAction.REFUNDALL)} variant="destructive" disabled={gw.ticketList.length === 0}>Refund All
                  Tickets</Button>
                {/*TODO show but disable these buttons if there are no tickets bought */}
              </>}
            {gw.status != GiveawayStatus.CREATE && <Button
              onClick={() => onAction(gw.status == GiveawayStatus.ARCHIVED ? GiveawayAction.UNARCHIVE : GiveawayAction.ARCHIVE)}
              variant="default">{gw.status == GiveawayStatus.ARCHIVED ? "Unarchive" : "Archive"}</Button>
            }
            <Button onClick={handleSubmit(onSave)} variant="default">Save</Button>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
}