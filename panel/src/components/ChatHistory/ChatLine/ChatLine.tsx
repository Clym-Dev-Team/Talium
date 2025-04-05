import {format} from "date-fns";
import {Message} from "@c/ChatHistory/Pane/Message.ts";
import "./ChatLine.css"

export interface ChatLineProps {
  message: Message
}

export default function ChatLine(props: ChatLineProps) {
  const date = new Date(props.message.timeStamp);
  const dateChat = format(date, "hh:MM:ss")
  const dateHint = format(date, "dd.mm.yyyy hh:MM:ss");
  return (
    <div className="message">
      <span className="message-timestamp std-font">
        <span className="timestamp-hint">{dateHint}</span>
        {dateChat}
      </span>
      <span className="message-user std-font" style={{color: props.message.userColor}}>{props.message.username}</span>
      <span className="std-font colon">: </span>
      <span className="message-content std-font">{props.message.message}</span>
    </div>
  )
}
