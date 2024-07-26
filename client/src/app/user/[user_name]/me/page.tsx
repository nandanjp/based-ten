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
    <div className="w-screen p-4">
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
        <TabsContent value="lists" className="p-6">
          <div className="text-3xl font-semibold mb-6">Lists</div>
          <div className="grid grid-cols-3 gap-4">
            {user_lists.isPending
              ? skel
              : user_lists.data?.response.map((l) => (
                  <ListCard
                    key={l.listname}
                    list_author={l.username!}
                    list_name={l.listname!}
                    list_type={l.list_type!}
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent value="likes" className="p-6">
          <div className="text-3xl font-semibold mb-6">Liked Lists</div>
          <div className="grid grid-cols-3 gap-4">
            {user_likes.isPending
              ? skel
              : user_likes.data?.response.map((l) => (
                  <ListCard
                    key={l.likingname.concat(l.listname)}
                    list_author={l.likingname}
                    list_name={l.listname}
                    list_type="anime" // TODO fix query}
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent value="followers" className="p-6">
          <FollowerList username={user_name} activeTab={activeTab} />
        </TabsContent>
        <TabsContent value="following" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Following</div>
            {user_following.isPending
              ? skel
              : user_following.data?.response.map((f) => (
                  <UserCard key={f.following} user_email={f.following} />
                ))}
          </div>
        </TabsContent>
        <TabsContent value="groups" className="p-6">
          <div className="flex justify-between pt-6">
            <div className="text-3xl font-semibold mb-6">Groups</div>
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
          <div className="grid grid-cols-3 gap-4 mt-4">
            {user_groups.isPending
              ? skel
              : user_groups.data?.response
                  ?.filter((g) => {
                    if (groupsShown === "joined")
                      return g.ownedby !== user_name;
                    return g.ownedby === user_name;
                  })
                  .map((g) => (
                    <GroupCard
                      key={g.groupname}
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
    </div>
  );
};
export default UserPage;
