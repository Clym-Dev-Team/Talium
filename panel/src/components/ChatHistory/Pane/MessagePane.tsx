import {useState, useRef, useEffect} from 'react';
import {Message} from "./Message.ts";
import {loadMore} from "./getMessages.ts";
import ChatLine from "../ChatLine/ChatLine.tsx";
import ChatHistoryQuery from "../Query/ChatHistoryQuery.tsx";
import "./MessagePane.css"


function InfiniteScroll() {
  const [items] = useState<Message[]>([]);
  const [earliestTimeStamp, setEarliestTimeStamp] = useState(6406316591700)
  const [query, setQuery] = useState("");
  const containerRef = useRef<HTMLDivElement>(null);
  const previousScrollPosition = useRef(0);

  function getEarlier(timeStamp: number) {
    const promise = loadMore(timeStamp);
    promise.then(value => {
      items.push(...value);
      // setItems(value)
      setEarliestTimeStamp(value[value.length - 1].timeStamp)
      if (containerRef.current != null) {
        containerRef.current.scrollTop = previousScrollPosition.current
      }
    })
    promise.catch(reason => console.log(reason))
  }

  function handleQuery(s: string) {
    setQuery(s);
  }

  useEffect(() => {
    // console.log('Container Ref:', containerRef.current);

    const handleScroll = () => {
      if (containerRef.current == null) {
        return;
      }
      // Save the current scroll position
      previousScrollPosition.current = containerRef.current.scrollTop;

      // Check if user has scrolled to the bottom
      if (containerRef.current.scrollTop + containerRef.current.clientHeight >= containerRef.current.scrollHeight) {
        getEarlier(earliestTimeStamp);
      }
    };

    if (containerRef.current != null) {
      containerRef.current.addEventListener('scroll', handleScroll);
    }
    return () => {
      // if (containerRef.current != null) {
        // containerRef.current.removeEventListener('scroll', handleScroll);
      // }
    };
  }, [items]); // Re-run effect when items change

  useEffect(() => {
    if (items.length == 0) {
      getEarlier(earliestTimeStamp);
    }
  }, []);

  return (
    <div className="message-pane">
      <div className="chat_history_header std-font">
        <ChatHistoryQuery value={query} onChange={handleQuery}/>
        <span>CHAT-HISTORY</span>
      </div>
      <div ref={containerRef} style={{height: '400px', overflowY: 'scroll'}}>
        {/*{items.map((item, index) => (*/}
        {/*  <div key={index}>/!* Render item content here *!/</div>*/}
        {/*))}*/}
        {items.map((message, index) => <ChatLine key={index} message={message}/>)}
      </div>
    </div>
  );
}

export default InfiniteScroll;
