import {Select, SelectContent, SelectItem, SelectTrigger} from "@shadcn/select.tsx";
import VLabel from "@s/VLabel.tsx";

export default function GWPolicySelector() {
  return <VLabel i18nFieldId="Giveaway Policy"><Select>
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

}