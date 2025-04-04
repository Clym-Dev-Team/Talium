import {GiveawayStatus} from "./Giveaway.ts";

export interface GiveawayPreview {
  id: string,
  title: string,
  notes: string,
  status: GiveawayStatus,
}