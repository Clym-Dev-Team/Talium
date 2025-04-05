import {Button} from "@shadcn/button.tsx";
import IconSave from "@i/IconSave.tsx";
import InfoData from "@s/InfoData.tsx";

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
      <InfoData i18nFieldId={"giveaway.edit.createdAt"} data={props.createdAt?.toISOString()}/>
      <InfoData i18nFieldId={"giveaway.edit.lastUpdatedAt"} data={props.lastUpdatedAt?.toISOString()}/>
    </div>
  </div>;
}