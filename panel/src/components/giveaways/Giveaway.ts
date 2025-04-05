export enum GiveawayStatus {
  CREATED,
  RUNNING,
  PAUSED,
  ARCHIVED,
}

export interface Giveaway {
  id: string,
  title: string,
  notes: string,
  createdAt: Date;
  lastUpdatedAt: Date;
  commandPattern: string,
  status: GiveawayStatus,
  autoStartTime?: Date,
  autoCloseTime?: Date,
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