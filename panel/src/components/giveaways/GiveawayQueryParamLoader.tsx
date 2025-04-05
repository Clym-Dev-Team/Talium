import GiveawayEditPage from "@c/giveaways/editPage/GiveawayEditPage.tsx";
import {useParams} from "react-router-dom";
import useData from "@s/useData.ts";
import {Giveaway, GiveawayJSON} from "@c/giveaways/Giveaway.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";

export default function GiveawayQueryParamLoader() {
  let {gwId} = useParams();
  const {data, loading} = useData<GiveawayJSON | undefined>("/giveaway/get/" + gwId, "Giveaway", undefined)
    //TODO load gw from id in url path
  if (loading) {
    return <Loader/>
  }
  if (data == undefined) {
    return "Error: Loading data failed, returned value undefined"
  }
  let gw: Giveaway = {
    status: data.status,
    title: data.title,
    maxTickets: data.maxTickets,
    notes: data.notes,
    ticketCost: data.ticketCost,
    autoStartTime: data.autoStartTime ? new Date(data.autoStartTime) : undefined,
    autoCloseTime: data.autoCloseTime ? new Date(data.autoCloseTime) : undefined,
    lastUpdatedAt: new Date(data.lastUpdatedAt),
    createdAt: new Date(data.createdAt),
    commandPattern: data.commandPattern,
    allowUserRedraw: data.allowUserRedraw,
    announceWinnerInChat: data.announceWinnerInChat,
    id: data.id,
    ticketList: data.ticketList,
    winnerList: data.winnerList,
  }
  return <GiveawayEditPage initialData={gw}/>
}