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
  const [avatarPreview, setAvatarPreview] = useState(companionData.avatar_path || companionAvatar);

  const userDataContext = useUserData();
  const userData: UserData = userDataContext?.userData ?? {} as UserData;
  const [userFormData, setUserFormData] = useState<UserData>(userData);

  const configContext = useConfigData();
  const configData: ConfigInterface = configContext?.config ?? {} as ConfigInterface;
  const [configFormData, setConfigFormData] = useState<ConfigInterface>(configData);

  const { refreshMessages, resetStart } = useMessages();

  const handleCompanionSave = async () => {
    if (companionFormData) {
      await updateCompanionData(companionFormData);
    }
  };

  const handleUserSave = async () => {
    if (userFormData) {
      await updateUserData(userFormData);
    }
  };

  const handleConfigSave = async () => {
    if (configFormData) {
      await updateConfigData(configFormData);
    }
  };

  const handleAvatarChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setAvatarFile(selectedFile);
      setAvatarPreview(URL.createObjectURL(selectedFile));
    }
  };

  useEffect(() => {
    if (companionDataContext) {
      setCompanionFormData(companionDataContext.companionData as CompanionData);
    }
  }, [companionDataContext?.companionData]);

  const handleAvatarUpload = async () => {
    if (avatarFile) {
      try {
        const formData = new FormData();
        formData.append("avatar", avatarFile);
        const response = await fetch("/api/companion/avatar", {
          method: "POST",
          headers: {
            'Content-Type': 'image/png',
        },
          body: avatarFile,
        });
        if (response.ok) {
          toast.success("Companion avatar changed successfully!");
          companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to change companion avatar");
          console.error("Failed to change companion avatar");
        }
      } catch (error) {
        console.error("Error uploading avatar:", error);
        toast.error(`Error uploading avatar: ${error}`);
      }
    } else {
      toast.warning("Please select an avatar file to upload");
      console.warn("Please select an avatar file to upload");
    }
  };

  const [characterCardFile, setCharacterCardFile] = useState<File | null>(null);

  const handleCharacterCardChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setCharacterCardFile(selectedFile);
    }
  };

  const handleCharacterCardUpload = async () => {
    if (characterCardFile) {
      try {
        const formData = new FormData();
        formData.append("character_card", characterCardFile);
        const response = await fetch("/api/companion/card", {
          method: "POST",
          headers: {
            'Content-Type': 'image/png',
        },
          body: characterCardFile,
        });
        if (response.ok) {
          toast.success("Companion card uploaded successfully!");
          await companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to upload character card");
          console.error("Failed to upload character card");
        }
      } catch (error) {
        console.error("Error uploading character card:", error);
        toast.error(`Error uploading character card: ${error}`);
      }
    } else {
      toast.warning("Please select an character card (.png) file to upload");
      console.warn("Please select an character card (.png) file to upload");
    }
  };

  const [characterJsonFile, setCharacterJsonFile] = useState<File | null>(null);

  const handleCharacterJsonChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setCharacterJsonFile(selectedFile);
    }
  };

  const handleCharacterJsonUpload = async () => {
    if (characterJsonFile) {
      try {
        const response = await fetch("/api/companion/characterJson", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: characterJsonFile,
        });
        if (response.ok) {
          toast.success("Character JSON uploaded successfully!");
          await companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to upload character JSON");
        }
      } catch (error) {
        console.error("Error uploading character JSON:", error);
        toast.error(`Error uploading character JSON: ${error}`);
      }
    } else {
      toast.warning("Please select a character JSON file to upload");
    }
  };

  const handleEraseDialogueTuning = async () => {
    try {
      const response = await fetch("/api/memory/dialogueTuning", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Character dialogue tuning cleared successfully!");
      } else {
        toast.error("Failed to erase dialogue tuning");
        console.error("Failed to erase dialogue tuning");
      }
    } catch (error) {
      toast.error(`Error while erasing dialogue tuning: ${error}`);
      console.error("Error while erasing dialogue tuning:", error);
    }
  };

  const handleEraseLongTerm = async () => {
    try {
      const response = await fetch("/api/memory/longTerm", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Long term memory cleared successfully!");
      } else {
        toast.error("Failed to erase long term memory");
        console.error("Failed to erase long term memory");
      }
    } catch (error) {
      toast.error(`Error while erasing long term memory: ${error}`);
      console.error("Error while erasing long term memory:", error);
    }
  };

  const handleClearMessages = async () => {
    try {
      const response = await fetch("/api/message", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Chat log cleared successfully!");
        resetStart();
        refreshMessages();
      } else {
        toast.error("Failed to clear chat log");
        console.error("Failed to clear chat log");
      }
    } catch (error) {
      toast.error(`Error while clearing chat log: ${error}`);
      console.error("Error while clearing chat log:", error);
    }
  };

  const handleExportCharacterJson = async () => {
    try {
      const response = await fetch("/api/companion/characterJson");
      if (response.ok) {
        const json = await response.json();
        const jsonString = JSON.stringify(json);
        const blob = new Blob([jsonString], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
     