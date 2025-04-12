import {ScrollArea} from "@c/ui/scroll-area.tsx";
import WinnerCard from "@c/giveaways/editPage/ticketCards/WinnerCard.tsx";
import TicketCard from "@c/giveaways/editPage/ticketCards/TicketCard.tsx";

export function TicketResultsPanel() {
  return <div className="winnersListCard">
    <ScrollArea className="winnersListScrollArea">
      <div className="winnerList">
        <div className="sectionTitle">
          <h3 className="sectionTitleText">Winner:</h3>
          <span className="legend">
            <span>Username</span>
            <span>Actions</span>
          </span>
        </div>
        <WinnerCard userId="" username="testUser82834834"/>
        <WinnerCard userId="" username="hdh82"/>
        <WinnerCard userId="" username="cookie"/>
        <WinnerCard userId="" username="supie"/>
        <WinnerCard userId="" username="clym"/>
      </div>
      <hr/>
      <div className="ticketList">
        <div className="sectionTitle">
          <h3 className="sectionTitleText">Submitted Tickets:</h3>
          <span className="legend">
            <span>Username</span>
            <span>Ticket Amount</span>
          </span>
        </div>
        <TicketCard tickets={98393} username="zuser23"/>
        <TicketCard tickets={1} username="user8239239"/>
        <TicketCard tickets={9} username="jfk"/>
      </div>
    </ScrollArea>
  </div>;
}