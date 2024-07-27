"use client";

import { GroupCard } from "@/components/blocks/GroupCard";
import { ListCard } from "@/components/blocks/ListCard";
import { UserCard } from "@/components/blocks/UserCard";
import { FollowerList } from "@/components/FollowerList";
import { buttonVariants } from "@/components/ui/button";
import GradientHeader from "@/components/ui/gradient-header";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { cn } from "@/lib/utils";
import { BadgePlus } from "lucide-react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";
import {
  useAllGroups,
  useCurrentUser,
  useUserFollowing,
  useUserLikes,
  useUsersLists,
} from "../../../../../services/queries";
import { HeroHighlight } from "@/components/animated/HeroHighlight";

const UserPage = () => {
  const { user_name } = useParams<{ user_name: string }>();
  const user_info = useCurrentUser();
  const user_lists = useUsersLists(user_name);
  const user_following = useUserFollowing(user_name);
  const user_likes = useUserLikes(user_name);
  const user_groups = useAllGroups();
  const [activeTab, setActiveTab] = useState("lists");
  const [groupsShown, setGroupsShown] = useState("all");

  useEffect(() => {
    user_info.refetch();
    user_lists.refetch();
    user_following.refetch();
    user_likes.refetch();
    user_groups.refetch();
  }, [user_name]);

  const skel = (
    <div className="flex items-center space-x-4">
      <Skeleton className="h-12 w-12 rounded-full" />
      <div className="space-y-2">
        <Skeleton className="h-4 w-[250px]" />
        <Skeleton className="h-4 w-[200px]" />
      </div>
    </div>
  );

  return (
    <HeroHighlight className="min-w-screen min-h-screen p-4">
      <GradientHeader title={user_info.data?.response?.username ?? "..."} />
      <Tabs
        defaultValue="lists"
        value={activeTab}
        onValueChange={setActiveTab}
        className="border-b"
      >
        <TabsList className="flex">
          <TabsTrigger value="lists">Lists</TabsTrigger>
          <TabsTrigger value="likes">Liked Lists</TabsTrigger>
          <TabsTrigger value="followers">Followers</TabsTrigger>
          <TabsTrigger value="following">Following</TabsTrigger>
          <TabsTrigger value="groups">Groups</TabsTrigger>
          <TabsTrigger value="account">Account</TabsTrigger>
        </TabsList>
        <TabsContent
          value="lists"
          className="flex flex-col items-center justify-center min-w-full min-h-full"
        >
          <h1 className="font-semibold px-3 py-2 text-5xl self-start text-center">
            Lists
          </h1>
          <div className="grid lg:grid-cols-3 gap-6 md:grid-cols-2 grid-cols-1">
            {user_lists.isPending
              ? skel
              : user_lists.data?.response?.map((l, i) => (
                  <ListCard
                    key={`${l.listname}-${i}`}
                    list_author={l.username!}
                    list_name={l.listname!}
                    list_type={l.list_type!}
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent
          value="likes"
          className="flex flex-col items-center justify-center min-w-full min-h-full"
        >
          <h1 className="font-semibold px-3 py-2 text-5xl self-start text-center">
            Liked Lists
          </h1>
          <div className="grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
            {user_likes.isPending
              ? skel
              : user_likes.data?.response?.map((l, i) => (
                  <ListCard
                    key={`${l.likername}-${i}`}
                    list_author={l.likingname}
                    list_name={l.listname}
                    list_type={"anime"} //TODO: fix-query
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent
          value="followers"
          className="flex flex-col gap-4 items-center justify-center min-w-full min-h-full"
        >
          <h1 className="text-5xl font-semibold self-start">Followers</h1>
          <FollowerList username={user_name} activeTab={activeTab} />
        </TabsContent>
        <TabsContent
          value="following"
          className="flex flex-col gap-4 justify-center items-center"
        >
          <h1 className="font-semibold px-3 py-2 text-5xl self-start text-center">
            Following
          </h1>
          <div className="grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
            {user_following.isPending
              ? skel
              : user_following.data?.response?.map((f, i) => (
                  <UserCard
                    key={`${f.following}-${i}`}
                    user_email={f.following}
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent
          value="groups"
          className="flex flex-col items-center justify-center min-w-full min-h-full"
        >
          <div className="flex md:flex-row sm:justify-between min-w-full flex-col justify-start">
            <h1 className="font-semibold px-3 py-2 text-5xl self-start text-center">
              Groups
            </h1>
            <Select defaultValue="owned" onValueChange={setGroupsShown}>
              <SelectTrigger className="w-[300px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="owned">Groups I own</SelectItem>
                <SelectItem value="joined">Groups I've joined</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="mb-4">
            <Link
              href={`/group/create_group`}
              className={cn(buttonVariants({ variant: "outline" }))}
            >
              <BadgePlus className="mr-2 h-4 w-4" />
              Create a new group
            </Link>
          </div>
          <hr />
          <div className="grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
            {user_groups.isPending
              ? skel
              : user_groups.data?.response
                  ?.filter((g) => {
                    if (groupsShown === "joined")
                      return g.ownedby !== user_name;
                    return g.ownedby === user_name;
                  })
                  .map((g, i) => (
                    <GroupCard
                      key={`${g.groupname}-${g.gid}-${i}`}
                      group_name={g.groupname}
                      group_id={g.gid}
                      owned_by={g.ownedby}
                    />
                  ))}
          </div>
        </TabsContent>
        <TabsContent value="account" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Account</div>
            <div>User email: {user_info.data?.response?.email ?? "..."}</div>
            <div>
              Created on:{" "}
              {user_info.data?.response?.createdat
                ? new Date(user_info.data.response.createdat).toLocaleString()
                : "..."}
            </div>
          </div>
        </TabsContent>
      </Tabs>
    </HeroHighlight>
  );
};
export default UserPage;
