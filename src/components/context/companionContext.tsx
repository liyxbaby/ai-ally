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
      data.avatar_path = data.avatar_path + "?timestamp=" + new Date().ge