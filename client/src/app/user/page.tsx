
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

const UserPage = () => {
    return (
        <div className="w-full xl mx-auto">
          <div className="bg-primary p-6">
            <div className="flex items-center gap-4">
              <div className="grid gap-1">
                <div className="text-xxl font-bold text-primary-foreground">John Doe</div>
                <div className="text-sm text-primary-foreground/80">john@example.com</div>
              </div>
            </div>
          </div>
          <Tabs defaultValue="lists" className="border-b">
            <TabsList className="flex">
              <TabsTrigger value="lists">My Lists</TabsTrigger>
              <TabsTrigger value="likes">Likes</TabsTrigger>
              <TabsTrigger value="following">Following</TabsTrigger>
              <TabsTrigger value="account">Account</TabsTrigger>
            </TabsList>
            <TabsContent value="profile" className="p-6">
              <div className="grid gap-4">
                <p>my lists</p>
              </div>
            </TabsContent>
            <TabsContent value="lists" className="p-6">
              <div className="grid gap-4">
                <p>lists</p>
              </div>
            </TabsContent>
            <TabsContent value="following" className="p-6">
              <div className="grid gap-4">
                <p>following</p>  
              </div>
            </TabsContent>
            <TabsContent value="account" className="p-6">
              <div className="grid gap-4">
                <p>account</p>  
              </div>
            </TabsContent>
          </Tabs>
        </div>
      )
}
export default UserPage;