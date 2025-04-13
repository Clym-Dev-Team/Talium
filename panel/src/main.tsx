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
import {PopoutProvider} from "@s/popoutProvider/PopoutProvider.tsx";
import i18n from "./i18n.ts";
import './index.css'
import './main.css'
import "./i18n";
import PublicConfigProvider from "@s/PublicConfigProvider.tsx";


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
          <PopoutProvider>
            <PublicConfigProvider>
            <RouterProvider router={router}/>
            </PublicConfigProvider>
          </PopoutProvider>
        </TooltipProvider>
    </I18nextProvider>
  </React.StrictMode>,
)