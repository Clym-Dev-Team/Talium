import GiveawayEditPage from "@c/giveaways/editPage/GiveawayEditPage.tsx";
import {useParams} from "react-router-dom";
import useData from "@s/useData.ts";
import {GiveawayJSON, parseJsonToGiveaway} from "@c/giveaways/Giveaway.ts";
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
  let gw = parseJsonToGiveaway(data);
  return <GiveawayEditPage initialData={gw}/>
}