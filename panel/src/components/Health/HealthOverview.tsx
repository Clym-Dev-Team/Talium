import useData from "@s/useData.ts";
import Loader from "@s/loadingSpinner/Loader.tsx";
import {InputHealth} from "./InputHealth.ts";
import HealthTile from "./HealthTile.tsx";
import "./HealthOverview.css"

export default function HealthOverview() {
  const {loading, data} = useData<InputHealth[]>("/health/json", "Health", [])

  if (loading) {
    return <Loader/>
  }

  return <div className="healthList">
    {data?.map(input => <HealthTile input={input}/>)}
  </div>
}