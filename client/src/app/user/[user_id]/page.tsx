"use client";

import { ListCard } from "@/components/blocks/ListCard/ListCard";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useParams } from "next/navigation";
import { useUsersLists } from "../../../../services/queries";

const UserPage = () => {
  const { user_id } = useParams<{ user_id: string }>();
  const user_lists = useUsersLists({ email: user_id });

  if (user_lists.isPending) {
    return <span>Loading....</span>;
  }

  if (user_lists.isError) {
    return <span>there was an error!</span>;
  }

  if (user_lists.isFetching) {
    return <span>data being fetched</span>;
  }

  if (!user_lists.data) {
    return <span>data not fetched</span>;
  }

  return (
    <div className="w-full xl mx-auto">
      <div className="bg-primary p-6">
        <div className="flex items-center pt-12 pl-6 pb-6">
          <div className="grid gap-1">
            <div className="text-4xl font-bold text-primary-foreground">
              John_Doe
            </div>
            <div className="text-sm text-primary-foreground/80">
              john@example.com
            </div>
          </div>
          <Tabs defaultValue="lists" className="border-b">
            <TabsList className="flex">
              <TabsTrigger value="lists">My Lists</TabsTrigger>
              <TabsTrigger value="likes">Liked Lists</TabsTrigger>
              <TabsTrigger value="followers">Followers</TabsTrigger>
              <TabsTrigger value="following">Following</TabsTrigger>
              <TabsTrigger value="account">Account</TabsTrigger>
            </TabsList>
            <TabsContent value="lists" className="p-6">
              <div className="grid gap-4">
                <div className="text-3xl font-semibold">My Lists</div>
                {user_lists.data.lists?.map((l) => (
                  <ListCard
                    key={l.email}
                    list_author={l.email!}
                    list_name={l.list_name!}
                    list_type={l.list_type!}
                  />
                ))}
              </div>
            </TabsContent>
            <TabsContent value="likes" className="p-6">
              <div className="grid gap-4">
                <div className="text-3xl font-semibold">Liked Lists</div>
              </div>
            </TabsContent>
            <TabsContent value="followers" className="p-6">
              <div className="grid gap-4">
                <div className="text-3xl font-semibold">Followers</div>
              </div>
            </TabsContent>
            <TabsContent value="following" className="p-6">
              <div className="grid gap-4">
                <div className="text-3xl font-semibold">Following</div>
              </div>
            </TabsContent>
            <TabsContent value="account" className="p-6">
              <div className="grid gap-4">
                <div className="text-3xl font-semibold">Account</div>
              </div>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  );
};
export default UserPage;
