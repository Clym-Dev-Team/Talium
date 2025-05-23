import {useEffect, useState} from "react";
import {useToast} from "../../../@shadcn/components/ui/use-toast.ts";
import {InputHealth} from "./InputHealth.ts";
import {getAllHealthStatuses} from "./HealthClient.ts";
import HealthTile from "./HealthTile.tsx";
import Loader from "../../common/LoadingSpinner/Loader.tsx";
import "./HealthOverview.css"

export default function HealthOverview() {
  const [loading, setLoading] = useState(true);
  const [health, setHealth] = useState<InputHealth[]>([])
  const toast = useToast();

  useEffect(() => {
    getAllHealthStatuses()
      .then(r => setHealth(r))
      .then(() => setLoading(false))
      .catch(reason => toast.toast(
        {className: "toast toast-failure", title: "ERROR Loading Health", description: reason.toString()}))
      .finally(() => setLoading(false));
  }, [])

  if (loading) {
    return <Loader/>
  }

  return <div className="healthList">
    {health?.map(input => <HealthTile input={input}/>)}
  </div>
}