import {Message} from "./Message.ts";
import {HISTORY_BACKEND_ADDR} from "@/main.tsx";

export async function loadMore(timeStamp: number): Promise<Message[]> {
  console.log("fetching messages...");
  const response = await fetch(
    `${HISTORY_BACKEND_ADDR}/messages/earlier?timeStamp=${timeStamp}`
  );
  return await response.json() as Message[];
}