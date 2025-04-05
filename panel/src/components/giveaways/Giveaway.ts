export enum GiveawayStatus {
  CREATED,
  RUNNING,
  PAUSED,
  ARCHIVED,
  AWAITING_CONFIRMATION
}

export interface Giveaway {
  id: string,
  title: string,
  notes: string,
  createdAt: Date;
  lastUpdatedAt: Date;
  commandPattern: string,
  status: GiveawayStatus,
  startTime?: number,
  endTime?: number,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
  // gwPolicy: string,
  // imageUrl?: string,
  // publicDescription?: string,
  // timerEnable: boolean,
  // timerGroupId?: string,
  // timerTemplate: string,
  // timerInterval: number,
}