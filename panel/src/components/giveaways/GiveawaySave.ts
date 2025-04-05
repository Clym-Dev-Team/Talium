
export interface GiveawaySave {
  commandPattern: string,
  title: string,
  notes?: string,
  autoStartTime?: Date,
  autoCloseTime?: Date,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
}