export enum GiveawayStatus {
  CREATED,
  RUNNING,
  PAUSED,
  ARCHIVED,
  AWAITING_CONFIRMATION
}

export interface Giveaway {
  id: string,
  commandPattern: string,
  title: string,
  notes?: string,
  startTime?: number,
  endTime?: number,
  status: GiveawayStatus,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
  gwPolicy: string,
  imageUrl?: string,
  publicDescription?: string,
  timerEnable: boolean,
  timerGroupId?: string,
  timerTemplate: string,
  timerInterval: number,
}