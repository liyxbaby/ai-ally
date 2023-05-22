import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { CompanionData } from '../interfaces/CompanionData';
import { toast } from "sonner";

interface CompanionDataProviderProps {
  children: ReactNode;
}

interface CompanionDataContextType {
  companionData: CompanionData | null;
  refreshCompanionData: () => void;
}

export const CompanionDataContext = createContext<CompanionDataContextType | null>(null);

export const CompanionDataProvider: React.FC<CompanionDataProviderProps> = ({ children }) => {
  const [companionData, setCompanionData] = useState<CompanionData | null>(null);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    fetchCompanionData().then((data: CompanionData) => {
      data.avatar_path = data.avatar_path + "?timestamp=" + new Date().getTime();
      setCompanionData(data);
    });
  }, [refreshData]);

  const fetchCompanionData = async () => {
    try {
      const response = await fetch('/api/companion');
      if (!response.ok) {
        throw new Error('');
      }
      const data = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      toast.error(`Error while fetching companion data: ${error}`);
      return null;
    }
  };

  const refreshCompanionData = () => {
    setRefreshData(!refreshData);
  };



  return (
    <CompanionDataContext.Provider value={{companionData, refreshCompanionData}}>
      {children}
    </Com