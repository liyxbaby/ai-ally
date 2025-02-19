
import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar";
import { ModeToggle } from "./mode-toggle";
import { EditDataPopup } from "./editData/EditDataPopup";
import { MessageScroll } from "./message/MessageScroll";
import { Textarea } from "./ui/textarea";
import { Menu, SendHorizontal } from "lucide-react";
import { Button } from "./ui/button";

import companionAvatar from "../assets/companion_avatar.jpg";

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu"
import { useCompanionData } from "./context/companionContext";
import { CompanionData } from "./interfaces/CompanionData";
import { useMessages } from "./context/messageContext";
import { useState } from "react";
import { toast } from "sonner";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "./ui/tooltip";

const ChatWindow = () => {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;

  const { refreshMessages, pushMessage } = useMessages();

  const [userMessage, setUserMessage] = useState('');
  const [companionMessage, setCompanionMessage] = useState('');
  const [isImpersonating, setIsImpersonating] = useState(false);
  const [prevUserMessage, setPrevUserMessage] = useState('');

  const handleMessageChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (isImpersonating) {
      setCompanionMessage(event.target.value);
    } else {
      setUserMessage(event.target.value);
    }
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      isImpersonating ? sendMessageAsAi() : promptMessage();
    }
  };

  const promptMessage = async () => {
    try {
      const sendPromise = fetch('/api/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ prompt: userMessage }),
      });
  
      const clearPromise = new Promise<void>(resolve => {
        setUserMessage('');
        resolve();
      });
  
      const pushSentMessagePromise = new Promise<void>(resolve => {
        pushMessage({
          id: -1,
          ai: false,
          content: userMessage,
          created_at: "now",
        });
        pushMessage({
          id: -2,
          ai: true,
          content: `${companionData.name} is typing...`,
          created_at: "",
        })
        resolve();
      });
  
      await Promise.all([sendPromise, clearPromise, pushSentMessagePromise]);
      refreshMessages();
  
    } catch (error) {
      console.error('Error sending message:', error);
      toast.error(`Error while sending a message: ${error}`);
    }
  };

  const sendMessageAsAi = async () => {
    try {
      const sendPromise = await fetch('/api/message', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ ai: true, content: companionMessage }),
      });
      
      if (sendPromise.ok) {
        await refreshMessages();
        setUserMessage('');
        setCompanionMessage('');
        setIsImpersonating(false);
      }

    } catch (error) {
      console.error('Error sending message:', error);
      toast.error(`Error while sending a message: ${error}`);
    }
  };

  const toggleImpersonateMode = () => {
    setIsImpersonating(!isImpersonating);
    if (!isImpersonating) {
      setPrevUserMessage(userMessage);
      setUserMessage('');
    } else {
      setUserMessage(prevUserMessage);
    }
  };

    return (
        <>
        <div className='w-full flex justify-end'>
            <ModeToggle />
          </div>
          <div className='flex flex-row items-center gap-5'>
          <Avatar>
            <AvatarImage src={companionData.avatar_path || companionAvatar} alt="Companion Avatar" />
            <AvatarFallback>AI</AvatarFallback>
          </Avatar>
          <EditDataPopup />
          </div>
          <MessageScroll />
          <div className="flex flex-row w-full items-center gap-2">
          <DropdownMenu>
            <DropdownMenuTrigger>
            <Button variant="outline" size={"sm"}>
                <Menu />
            </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side="top">
                <DropdownMenuItem onClick={toggleImpersonateMode}>{isImpersonating ? 'Stop impersonating' : 'Impersonate'}</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
        <Textarea value={isImpersonating ? companionMessage : userMessage} onChange={handleMessageChange} cols={1} placeholder={isImpersonating ? `🥸 Type your message as ${companionData?.name}` : "Type your message"} onKeyDown={handleKeyDown} />
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button size={"sm"} onClick={() => {isImpersonating ? sendMessageAsAi() : promptMessage()}}><SendHorizontal /></Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>{isImpersonating ? `Send message as ${companionData?.name}` : "Send message"}</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
          </div>
        </>
    )
}

export default ChatWindow;