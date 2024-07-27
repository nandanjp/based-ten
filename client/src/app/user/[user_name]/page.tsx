"use client";

import { createFollow, deleteFollow } from "@/app/actions";
import { UserContext } from "@/app/context";
import { GroupCard } from "@/components/blocks/GroupCard";
import { ListCard } from "@/components/blocks/ListCard";
import { UserCard } from "@/components/blocks/UserCard";
import { Button } from "@/components/ui/button";
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
import { useParams } from "next/navigation";
import { useContext, useEffect, useState } from "react";
import {
  useAllGroups,
  useUserFollowers,
  useUserFollowing,
  useUserLikes,
  useUsersLists,
} from "../../../../services/queries";
import { HeroHighlight } from "@/components/animated/HeroHighlight";

const UserPage = () => {
  const { user_name } = useParams<{ user_name: string }>();
  const user_lists = useUsersLists(user_name);
  const user_following = useUserFollowing(user_name);
  const user_followers = useUserFollowers(user_name);
  const user_likes = useUserLikes(user_name);
  const user_groups = useAllGroups();
  const logged_in_user = useContext(UserContext);
  const [currentUserFollows, setCurrentUserFollows] = useState(false);
  const [groupsShown, setGroupsShown] = useState("all");

  useEffect(() => {
    user_lists.refetch();
    user_following.refetch();
    user_followers.refetch();
    user_likes.refetch();
    user_groups.refetch();
  }, [user_name]);

  useEffect(() => {
    if (user_followers.data && logged_in_user) {
      const isUserFollowing = user_followers.data.response.some(
        (follower) => follower.follower == logged_in_user.user?.username
      );
      setCurrentUserFollows(isUserFollowing);
    }
  }, [user_following, logged_in_user]);

  const onFollowButtonClick = async () => {
    if (currentUserFollows) {
      const response = await deleteFollow(user_name);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
    } else {
      const response = await createFollow(user_name);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
    }
    setCurrentUserFollows(!currentUserFollows);
    location.reload();
  };

  const followButtonVariant = currentUserFollows ? "secondary" : "default";
  const followButtonText = currentUserFollows ? "Following" : "Follow";

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
      <div className="flex flex-col gap-5 items-center justify-center pb-4 space-x-6">
        <h1 className="text-5xl font-bold p-4">{user_name}</h1>
        <Button
          variant={followButtonVariant}
          onClick={onFollowButtonClick}
          className="max-w-40"
        >
          {followButtonText}
        </Button>
      </div>
      <Tabs defaultValue="lists" className="border-b">
        <TabsList className="flex justify-evenly min-w-full">
          <TabsTrigger value="lists">Lists</TabsTrigger>
          <TabsTrigger value="likes">Liked Lists</TabsTrigger>
          <TabsTrigger value="followers">Followers</TabsTrigger>
          <TabsTrigger value="following">Following</TabsTrigger>
          <TabsTrigger value="groups">Groups</TabsTrigger>
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
          className="flex flex-col items-start justify-start min-w-full min-h-full"
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
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent
          value="followers"
          className="flex flex-col gap-6 justify-center items-center"
        >
          <h1 className="font-semibold px-3 py-2 text-5xl self-start text-center">
            Followers
          </h1>
          <div className="grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
            {user_followers.isPending
              ? skel
              : user_followers.data?.response?.map((f, i) => (
                  <UserCard
                    key={`${f.follower}-${i}`}
                    user_email={f.follower}
                  />
                ))}
          </div>
        </TabsContent>
        <TabsContent
          value="following"
          className="flex flex-col gap-6 justify-center items-center"
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
          className="flex flex-col gap-6 justify-center items-center"
        >
          <div className="flex md:flex-row sm:justify-between py-6 min-w-full flex-col justify-start">
            <div className="text-3xl font-semibold mb-6">Groups</div>
            <Select defaultValue="all" onValueChange={setGroupsShown}>
              <SelectTrigger className="w-[300px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All groups</SelectItem>
                <SelectItem value="owned">Owned groups</SelectItem>
                <SelectItem value="joined">Joined groups</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
            {user_groups.isPending
              ? skel
              : user_groups.data?.response
                  ?.filter((g) => {
                    if (groupsShown === "all") return true;
                    if (groupsShown === "owned") return g.ownedby === user_name;
                    if (groupsShown === "joined")
                      return g.ownedby !== user_name;
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
      </Tabs>
    </HeroHighlight>
  );
};
export default UserPage;
