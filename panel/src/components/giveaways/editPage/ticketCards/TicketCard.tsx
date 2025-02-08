import "./TicketCard.css"

export interface TicketCardProps {
    username: string,
    tickets: number,
}

export default function TicketCard({tickets, username}: TicketCardProps) {
  return <div className="gwResultCard ticketCard">
    <span>{username}</span>
    <span>×{tickets}</span>
  </div>
}