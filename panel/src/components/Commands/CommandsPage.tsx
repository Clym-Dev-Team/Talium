import {Tabs, TabsContent, TabsList, TabsTrigger} from "@shadcn/tabs.tsx";
import AllCommandList from "./commands/AllCommandList.tsx";
import UserCommandList from "./commands/UserCommandList.tsx";
import "./CommandPage.css"
import TemplateList from "./templates/TemplateList.tsx";

export default function CommandsPage() {
  return <Tabs defaultValue="commands" >
    <TabsList className="commandTabList">
      <TabsTrigger value="commands">User-Commands</TabsTrigger>
      <TabsTrigger value="templates">Templates</TabsTrigger>
      <TabsTrigger value="trigger">Internal Commands</TabsTrigger>
    </TabsList>
    <TabsContent value="commands">
      <UserCommandList/>
    </TabsContent>
    <TabsContent value="templates">
      <TemplateList/>
    </TabsContent>
    <TabsContent value="trigger">
      <AllCommandList/>
    </TabsContent>
  </Tabs>
}