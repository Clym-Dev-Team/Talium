import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "../../../@shadcn/components/ui/table.tsx";
import Loader from "../../common/LoadingSpinner/Loader.tsx";
import useData from "../../common/useData.ts";

interface LeaderboardDTO {
  twitchUserName: string,
  coins: number,
  watchtimeSeconds: number,
}

export default function WatchtitmeLeaderboard() {
  const {data, loading} = useData<LeaderboardDTO[]>("/watchtime/top", "WatchtimeLeaderboard", [])

  return <>
    <h1>Top 100 Leaderboard - Watchtime:</h1>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="tw-w-10">#</TableHead>
          <TableHead>Username</TableHead>
          <TableHead className="tw-w-32">Coins</TableHead>
          <TableHead>Seconds</TableHead>
          <TableHead className="tw-w-32">Days</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {!loading ? data.map((value, index) => (
          <TableRow key={index}>
            <TableCell>{index + 1}</TableCell>
            <TableCell>{value.twitchUserName}</TableCell>
            <TableCell>{value.coins}</TableCell>
            <TableCell>{value.watchtimeSeconds}</TableCell>
            <TableCell>{value.watchtimeSeconds / 86400}</TableCell>
          </TableRow>
        )) : <Loader/>}
      </TableBody>
    </Table>
  </>
}