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

export const UserDataProvider: React.FC<UserDataProviderProps> = ({ children }) => {
  const [userData, setUserData] = useState<UserData | null>(null);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    fetchUserData().then((data) => {
      setUserData(data);
    });
  }, [refreshData]);

  const fetchUserData = async () => {
    try {
      const response = await fetch('/api/user');
      if (!response.ok) {
        throw new Error('');
      }
      const data = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      toast.error(`Error while fetching user data: ${error}`);
      return null;
    }
 