import { useState, useEffect, useRef } from 'react';
import { ScrollArea } from "@/components/ui/scroll-area";
import { Message } from "./Message";
import { useMessages } from "../context/messageContext";

export function MessageScroll() {
  const scrollRef = useRef<HTMLDivElement>(null);
  const { messages, loadMoreMessages } = useMessages();
  const [hasMoreMessages, setHasMoreMessages] = useState<boolean>(true);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight - scrollRef.current.clientHeight;
    }
  }, [messages]);

  useEffect(() => {
    setHasMoreMessages(messages.length >= 50);
  }, [messages]);

  const handleLoadMore = () => {
    loadMoreMessages();
  };

  return (
    <ScrollArea
      ref={scrollRef}
      className="h-[70vh] md:h-[82vh] w-