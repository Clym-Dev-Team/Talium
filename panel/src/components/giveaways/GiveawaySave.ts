
export interface GiveawaySave {
  commandPattern: string,
  title: string,
  notes?: string,
  startTime?: number,
  endTime?: number,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
}