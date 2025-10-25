import {Message} from "./Message.ts";

import {history_backend_addr} from "@/static_config.tsx";

export async function loadMore(timeStamp: number): Promise<Message[]> {
  console.log("fetching messages...");
  const response = await fetch(
    `${history_backend_addr()}/messages/earlier?timeStamp=${timeStamp}`
  );
  return await response.json() as Message[];
}
