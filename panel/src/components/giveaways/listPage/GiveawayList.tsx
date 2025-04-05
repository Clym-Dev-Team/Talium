import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@shadcn/accordion.tsx";
import GiveawayTile from "./GiveawayTile.tsx";
import {ScrollArea} from "@c/ui/scroll-area.tsx";
import {GiveawayPreview} from "@c/giveaways/GiveawayPreview.ts";
import useData from "@s/useData.ts";
import Loader from "@s/LoadingSpinner/Loader.tsx";

export default function GiveawayList() {
  const {loading, data} = useData<GiveawayPreview[]>("/giveaway/listActive", "Giveaway", [])

  return <ScrollArea className="giveawayList">
    <div className="activeList">
      {!loading ? data.map((preview) => <GiveawayTile preview={preview}/>):<Loader/>}
    </div>
    <Accordion type="single">
      <AccordionItem value="item-1">
        <AccordionTrigger>Archived Giveaways</AccordionTrigger>
        <AccordionContent className="archivedList">
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </ScrollArea>;
}