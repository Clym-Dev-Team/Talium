import React from 'react'
import {createBrowserRouter, createRoutesFromChildren, Outlet, Route, RouterProvider} from "react-router-dom";
import {I18nextProvider} from "react-i18next";
import ReactDOM from 'react-dom/client'
import {TooltipProvider} from "@shadcn/tooltip.tsx";
import {Toaster} from "@shadcn/toaster.tsx";
import WatchtimePage from "@c/watchtime/WatchtimePage.tsx";
import AccountsPage from "@c/panelAccounts/AccountsPage.tsx";
import OauthSetup from "@c/OauthManager/OauthSetup.tsx";
import OauthResult from "@c/OauthManager/OauthResult.tsx";
import TwitchNavMenu from "@c/NavMenuTwitch/TwitchNavMenu.tsx";
import TokenRemover from "@c/Login/TokenRemover.tsx";
import LoginPage from "@c/Login/LoginPage.tsx";
import HealthOverview from "@c/Health/HealthOverview.tsx";
import GiveawayListPage from "@c/giveaways/listPage/GiveawayListPage.tsx";
import GiveawayQueryParamLoader from "@c/giveaways/GiveawayQueryParamLoader.tsx";
import CommandsPage from "@c/Commands/CommandsPage.tsx";
import MessagePane from "@c/ChatHistory/Pane/MessagePane.tsx";
import i18n from "./i18n.ts";
import './index.css'
import './main.css'
import "./i18n";

export const PANEL_BASE_URL = import.meta.env.VITE_PANEL_BASE_URL;
export const BOT_BACKEND_ADDR = import.meta.env.VITE_BOT_BACKEND_ADDR
export const HISTORY_BACKEND_ADDR = import.meta.env.VITE_HISTORY_BACKEND_ADDR
export const TWITCH_CLIENT_ID = import.meta.env.VITE_TWITCH_CLIENT_ID

const router = createBrowserRouter(
  createRoutesFromChildren(
    <>
      <Route path="/twitchToken" element={<TokenRemover/>}/>
      <Route element={
        <LoginPage>
          <TwitchNavMenu/>
          <div className="contentRoot dark">
            <Outlet/>
            <Toaster/>
          </div>
        </LoginPage>
      }>
        <Route path="*" element={<p>Diese seite gibt es nicht</p>}/>
        <Route path="/" element={<p>Diese seite gibt es nicht</p>}/>
        <Route path="/history" element={<MessagePane/>}/>
        <Route path="/commands" element={<CommandsPage/>}/>
        <Route path="/health" element={<HealthOverview/>}/>
        <Route path="/oauth" element={<OauthSetup/>}/>
        <Route path="/oauth/result" element={<OauthResult/>}/>
        <Route path="/accounts" element={<AccountsPage/>}/>
        <Route path="/giveaways" element={<GiveawayListPage/>}/>
        <Route path="/giveawayEdit/:gwId" element={<GiveawayQueryParamLoader/>}/>
        <Route path="/watchtime" element={<WatchtimePage/>}/>
      </Route>
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