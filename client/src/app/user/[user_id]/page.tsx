'use client';

import { ListCard } from '@/components/blocks/ListCard';
import { UserCard } from '@/components/blocks/UserCard';
import { GroupCard } from '@/components/blocks/GroupCard';
import { UserCardFollowBack } from '@/components/blocks/UserCardFollowBack';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useParams } from 'next/navigation';
import {
  useUsersLists,
  useUser,
  useUserFollowing,
  useUserFollowers,
  useUserLikes,
  useUserGroups,
} from '../../../../services/queries';
import GradientHeader from '@/components/ui/gradient-header';

const UserPage = () => {
  const { user_id } = useParams<{ user_id: string }>();
  const user_info = useUser({ email: user_id });
  const user_lists = useUsersLists({ email: user_id });
  const user_following = useUserFollowing({ email: user_id });
  const user_followers = useUserFollowers({ email: user_id });
  const user_likes = useUserLikes({ email: user_id });
  const user_groups = useUserGroups({ email: user_id });

  if (
    user_lists.isPending ||
    user_info.isPending ||
    user_following.isPending ||
    user_followers.isPending ||
    user_likes.isPending ||
    user_groups.isPending
  ) {
    return <span>Loading....</span>;
  }

  if (
    user_lists.isError ||
    user_info.isError ||
    user_following.isError ||
    user_followers.isError ||
    user_likes.isError ||
    user_groups.isError
  ) {
    return <span>there was an error!</span>;
  }

  if (user_lists.isFetching || user_info.isFetching || user_likes.isFetching || user_followers.isFetching || user_following.isFetching || user_groups.isFetching) {
    return <span>data being fetched</span>;
  }

  if (!user_lists.data) {
    return <span>data not fetched</span>;
  }

  console.log(user_info.data);

  return (
    <div className="w-screen">
      <GradientHeader 
        title={user_info.data.response?.user_name} 
        subtitle={user_info.data.response?.email} 
      />
      <Tabs defaultValue="lists" className="border-b">
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
            {user_lists.data.response?.map((l) => (
              <ListCard
                key={l.user_name}
                list_author={l.user_name!}
                list_name={l.list_name!}
                list_type={l.list_type!}
              />
            ))}
          </div>
        </TabsContent>
        <TabsContent value="likes" className="p-6">
          <div className="text-3xl font-semibold mb-6">Liked Lists</div>
            <div className="grid grid-cols-3 gap-4">
            {user_likes.data.response?.map((l) => (
              <ListCard
                key={l.liking_name.concat(l.list_name)}
                list_author={l.liking_name}
                list_name={l.list_name}
                list_type="anime" // TODO fix query
              />
            ))}
          </div>
        </TabsContent>
        <TabsContent value="followers" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Followers</div>
            {user_followers.data.response?.map((f) => (
              <UserCardFollowBack
                key={f.follower}
                follower_email={f.follower}
                follows_back={f.follows_back}
              />
            ))}
          </div>
        </TabsContent>
        <TabsContent value="following" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Following</div>
            {user_following.data.response?.map((f) => (
              <UserCard key={f.following} user_email={f.following} />
            ))}
          </div>
        </TabsContent>
        <TabsContent value="groups" className="p-6">
        <div className="text-3xl font-semibold mb-6">Groups</div>
          <div className="grid grid-cols-3 gap-4">
            {user_groups.data.response?.map((g) => (
              <GroupCard
                key={g.group_name}
                group_name={g.group_name}
                group_id={g.gid}
                owned_by={g.owned_by}
              />
            ))}
          </div>
        </TabsContent>
        <TabsContent value="account" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Account</div>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
};
export default UserPage;
