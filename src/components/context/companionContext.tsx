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

export const CompanionDataProvider: React.FC<CompanionDataProviderProps> = ({ childr