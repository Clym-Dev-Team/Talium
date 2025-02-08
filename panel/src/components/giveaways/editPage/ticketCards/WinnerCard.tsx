import {useState} from "react";
import "./TicketCard.css"

export interface WinnerCardProps {
  username: string,
}

export default function WinnerCard(props: WinnerCardProps) {
  const [comment, setComment] = useState<string | undefined>(undefined);
  const randomBool = Math.random() > 0.5;
  if (randomBool) {
    setComment(window.crypto.randomUUID());
  }
  const [rejected, setRejected] = useState(Math.random() > 0.5)
  return <div className={ rejected ? "rejected gwResultCard winnerCard" : "gwResultCard winnerCard"}>
    <span className="username">{props.username}</span>
    {comment ? <span className="comment">{comment}</span> : ""}
  </div>
}