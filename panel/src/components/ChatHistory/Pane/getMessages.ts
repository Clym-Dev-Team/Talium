import {Message} from "./Message.ts";

export async function loadMore(timeStamp: number): Promise<Message[]> {
  console.log("fetching messages...");
  let chatHistoryBackend = "";
  const response = await fetch(
    `${chatHistoryBackend}/messages/earlier?timeStamp=${timeStamp}`
  );
  return await response.json() as Message[];
}