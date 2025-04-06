import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@shadcn/accordion.tsx";
import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import {ScrollArea} from "@c/ui/scroll-area.tsx";
import {GiveawayPreview} from "@c/giveaways/GiveawayPreview.ts";
import GiveawayTile from "./GiveawayTile.tsx";

export default function GiveawayList() {
  const {loading, data} = useData<GiveawayPreview[]>("/giveaway/listActive", "Giveaway", [])

  return <ScrollArea className="giveawayList">
    <div className="activeList">
      {!loading ? data.map((preview) => <GiveawayTile key={preview.id} preview={preview}/>):<Loader/>}
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