import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { CompanionData } from '../interfaces/CompanionData';
import { toast } from "sonner";

interface CompanionDataProviderProps {
  chil