export enum GiveawayStatus {
  CREATE, // Special status to tell GW Edit page that this is an object creation, and to disable certain actions
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
  autoStartTime: Date | null,
  autoCloseTime: Date | null,
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

export function parseJsonToGiveaway(data: GiveawayJSON): Giveaway {
  return {
    status: (GiveawayStatus as any)[data.status] as GiveawayStatus,
    title: data.title,
    maxTickets: data.maxTickets,
    notes: data.notes,
    ticketCost: data.ticketCost,
    autoStartTime: data.autoStartTime ? new Date(data.autoStartTime) : undefined,
    autoCloseTime: data.autoCloseTime ? new Date(data.autoCloseTime) : undefined,
    lastUpdatedAt: new Date(data.lastUpdatedAt),
    createdAt: new Date(data.createdAt),
    commandPattern: data.commandPattern,
    allowUserRedraw: data.allowUserRedraw,
    announceWinnerInChat: data.announceWinnerInChat,
    id: data.id,
    ticketList: data.ticketList ? data.ticketList : [],
    winnerList: data.winnerList ? data.winnerList : [],
  } as Giveaway;
}
