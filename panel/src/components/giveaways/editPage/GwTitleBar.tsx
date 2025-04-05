import {Button} from "@shadcn/button.tsx";
import IconSave from "@i/IconSave.tsx";
import InfoData from "@s/InfoData.tsx";
import {format} from "date-fns";

export interface GiveawayProps {
  id: string,
  title: string,
  createdAt: Date,
  lastUpdatedAt: Date,
  onSave: () => void,
}

export function GwTitleBar(props: GiveawayProps) {
  return <div className="tileBar">
    <Button variant="default" className="saveBtn" onClick={props.onSave}><IconSave/></Button>
    <span>Edit: {props.title}</span>
    <div className="infoBoxes">
      <InfoData i18nFieldId={"giveaway.edit.gwId"} data={props.id}/>
      <InfoData i18nFieldId={"giveaway.edit.createdAt"} data={format(props.createdAt, "dd.MM.yyyy-HH:mm")}/>
      <InfoData i18nFieldId={"giveaway.edit.lastUpdatedAt"} data={format(props.lastUpdatedAt, "dd.MM.yyyy-HH:mm")}/>
    </div>
  </div>;
}