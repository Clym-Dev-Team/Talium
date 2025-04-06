
export interface GiveawaySave {
  commandPattern: string,
  title: string,
  notes: string,
  autoStartTime: Date | null,
  autoCloseTime: Date | null,
  ticketCost: number,
  maxTickets: number,
  allowUserRedraw: boolean,
  announceWinnerInChat: boolean,
}