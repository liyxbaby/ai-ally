import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { UserData } from '../interfaces/UserData';
import { toast } from "sonner";

interface UserDataProviderProps {
  children: ReactNode;
}

interface UserDataContextType {
  userData: UserData | null;
  refreshUserData: () => void;
}

const UserDataContext = createContext<UserDataContextType | null>(null);

export const UserDataProvider: React.FC<UserDa