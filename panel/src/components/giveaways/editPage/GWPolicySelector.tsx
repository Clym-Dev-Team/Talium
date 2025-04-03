import VLabel from "../../../common/VerticalLabel/VLabel.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "../../../../@shadcn/components/ui/select.tsx";

export default function GWPolicySelector() {
  return <VLabel name="Giveaway Policy"><Select>
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