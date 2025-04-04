import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import './main.css'
import {createBrowserRouter, createRoutesFromChildren, Outlet, Route, RouterProvider} from "react-router-dom";
import MessagePane from "./components/ChatHistory/Pane/MessagePane.tsx";
import LoginPage from "./components/Login/LoginPage.tsx";
import {Toaster} from "../@shadcn/components/ui/toaster.tsx";
import TwitchNavMenu from "./components/NavMenuTwitch/TwitchNavMenu.tsx";
import HealthOverview from "./components/Health/HealthOverview.tsx";
import OauthSetup from "./components/OauthManager/OauthSetup.tsx";
import OauthResult from "./components/OauthManager/OauthResult.tsx";
import CommandsPage from "./components/Commands/CommandsPage.tsx";
import GiveawayListPage from "./components/giveaways/listPage/GiveawayListPage.tsx";
import GiveawayEditPage from "./components/giveaways/editPage/GiveawayEditPage.tsx";
import {TooltipProvider} from "../@shadcn/components/ui/tooltip.tsx";
import TokenRemover from "./components/Login/TokenRemover.tsx";
import AccountsPage from "./components/panelAccounts/AccountsPage.tsx";
import WatchtimePage from "./components/watchtime/WatchtimePage.tsx";
import {I18nextProvider} from "react-i18next";
import i18n from "./i18n.ts";
import "./i18n"; // import to initialize

export const PANEL_BASE_URL = import.meta.env.VITE_PANEL_BASE_URL;
export const BOT_BACKEND_ADDR = import.meta.env.VITE_BOT_BACKEND_ADDR
export const HISTORY_BACKEND_ADDR = import.meta.env.VITE_HISTORY_BACKEND_ADDR
export const TWITCH_CLIENT_ID = import.meta.env.VITE_TWITCH_CLIENT_ID

const router = createBrowserRouter(
  createRoutesFromChildren(
    <>
      <Route element={
        <LoginPage>
          <TwitchNavMenu/>
          <div className="contentRoot dark">
            <Outlet/>
            <Toaster/>
          </div>
        </LoginPage>
      }>
        <Route path="/history" element={<MessagePane/>}/>
        <Route path="*" element={<p>Diese seite gibt es nicht</p>}/>
        <Route path="/" element={<p>Diese seite gibt es nicht</p>}/>
        <Route path="/commands" element={<CommandsPage/>}/>
        <Route path="/health" element={<HealthOverview/>}/>
        <Route path="/oauth" element={<OauthSetup/>}/>
        <Route path="/oauth/result" element={<OauthResult/>}/>
        <Route path="/accounts" element={<AccountsPage/>}/>
        <Route path="/giveaways" element={<GiveawayListPage/>}/>
        <Route path="/giveawayEdit/*" element={<GiveawayEditPage/>}/>
        <Route path="/watchtime" element={<WatchtimePage/>}/>
      </Route>
      <Route path="/twitchToken" element={<TokenRemover/>}/>
    </>
  )
);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <I18nextProvider i18n={i18n}>
      <TooltipProvider>
        <RouterProvider router={router}/>
      </TooltipProvider>
    </I18nextProvider>
  </React.StrictMode>,
)