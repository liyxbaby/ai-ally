import './App.scss'

import { ThemeProvider } from "@/components/theme-provider"
import Footer from './components/Footer'
import ChatWindow from './components/ChatWindow'
import { MessagesProvider } from './components/context/messageContext'
import { UserDataProvider } from './components/context/userContext'
import { CompanionDataProvider } from './components/context/companionContext'
import { ConfigProvider } from './components/context/configContext'

import { Toaster