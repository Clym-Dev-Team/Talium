import GiveawayTile from "./GiveawayTile.tsx";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
} from "@shadcn/accordion.tsx";
import useData from "@/common/useData.ts";
import Loader from "@/common/LoadingSpinner/Loader.tsx";
import {ScrollArea} from "@/components/ui/scroll-area.tsx";
import {GiveawayPreview} from "@/components/giveaways/GiveawayPreview.ts";

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