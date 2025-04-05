import {InputHealth} from "./InputHealth.ts";
import HealthTile from "./HealthTile.tsx";
import useData from "@/common/useData.ts";
import Loader from "@/common/LoadingSpinner/Loader.tsx";
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