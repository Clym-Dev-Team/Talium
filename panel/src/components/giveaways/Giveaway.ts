export enum GiveawayStatus {
  CREATED,
  RUNNING,
  PAUSED,
  ARCHIVED,
}

export interface Entry {
  userId: string,
  userName: string,
  tickets: number
}

export interface Winner {
  userId: string,
  userName: string,
  rejected: boolean,
  comment?: string
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
  ticketList: Entry[],
  winnerList: Winner[],
  // gwPolicy: string,
  // imageUrl?: string,
  // publicDescription?: string,
  // timerEnable: boolean,
  // timerGroupId?: string,
  // timerTemplate: string,
  // timerInterval: number,
}

export interface GiveawayJSON {
  id: string,
  title: string,
  notes: string,
  createdAt: string;
  lastUpdatedAt: string;
  commandPattern: string,
  status: string,
  autoStartTime?: string,
  autoCloseTime?: string,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
  ticketList: Entry[],
  winnerList: Winner[],
}
