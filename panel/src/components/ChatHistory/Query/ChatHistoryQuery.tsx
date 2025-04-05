import {useState} from "react";
import "./ChatHistoryQuery.css"
import {Input} from "@shadcn/input.tsx";

interface props {
  value: string,
  onChange: (value: string) => void,
}

export default function ChatHistoryQuery(props: props) {
  const [value, setValue] = useState(props.value)

  return (
    <div className="chat_history_query">
      <Input type="text" placeholder="Suche" value={value} onSubmit={() => props.onChange(value)} onChange={ event => setValue(event.target.value)} />
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-6 h-6">
        <path strokeLinecap="round" strokeLinejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"/>
      </svg>
    </div>
  )
}