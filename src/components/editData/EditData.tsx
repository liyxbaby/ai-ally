import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "../ui/textarea"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Info } from "lucide-react"

import companionAvatar from "../../assets/companion_avatar.jpg";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { updateCompanionData,  useCompanionData } from "../context/companionContext"

import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { updateUserData, useUserData } from "../context/userContext"
import { updateConfigData, useConfigData } from "../context/configContext"
import { ConfigInterface, Device, PromptTemplate } from "../interfaces/Config"
import { useEffect, useState } from "react"
import { CompanionData } from "../interfaces/CompanionData"
import { UserData } from "../interfaces/UserData"
import { toast } from "sonner"
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "../ui/dialog"
import { useMessages } from "../context/messageContext"

export function EditData() {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;
  const [companionFormData, setCompanionFormData] = useState<CompanionData>(companionData);
  const [avatarFile, setAvatarFile] = useState<File | null>(null);
  const 