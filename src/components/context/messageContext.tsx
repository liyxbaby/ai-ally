import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { MessageInterface } from '../interfaces/Message';
import { toast } from "sonner";

interface MessagesProviderProps {
  children: ReactNode;
}

interface MessagesContextType {
  messages: MessageInterface[];
  refreshMessages: () => void;
  pushMessage: (message: MessageInterface) => void;
  loadMoreMessages: () => void;
  resetStart : () => void;
}

const MessagesContext = createContext<Message