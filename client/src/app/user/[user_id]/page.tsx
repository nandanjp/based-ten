'use client';

import { ListCard } from '@/components/blocks/ListCard';
import { UserCard } from '@/components/blocks/UserCard';
import { UserCardFollowBack } from '@/components/blocks/UserCardFollowBack';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useParams } from 'next/navigation';
import {
  useUsersLists,
  useUser,
  useUserFollowing,
  useUserFollowers,
  useUserLikes,
} from '../../../../services/queries';

const UserPage = () => {
  const { user_id } = useParams<{ user_id: string }>();
  const user_info = useUser({ email: user_id });
  const user_lists = useUsersLists({ email: user_id });
  const user_following = useUserFollowing({ email: user_id });
  const user_followers = useUserFollowers({ email: user_id });
  const user_likes = useUserLikes({ email: user_id });

  if (
    user_lists.isPending ||
    user_info.isPending ||
    user_following.isPending ||
    user_followers.isPending ||
    user_likes.isPending
  ) {
    return <span>Loading....</span>;
  }

  if (
    user_lists.isError ||
    user_info.isError ||
    user_following.isError ||
    user_followers.isError ||
    user_likes.isError
  ) {
    return <span>there was an error!</span>;
  }

  if (user_lists.isFetching || user_info.isFetching || user_likes.isFetching) {
    return <span>data being fetched</span>;
  }

  if (!user_lists.data) {
    return <span>data not fetched</span>;
  }

  console.log(user_info.data);

  return (
    <div className="w-screen">
      <div style={{
        height: '10rem',
        background: 'linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)',
        backgroundSize: '150% 150%',
        animation: 'gradientFlow 10s ease infinite',
      }}
      className="flex items-center justify-center flex-col">
        <div className="flex items-center  pt-12 pl-6 pb-6">
          <div className="grid gap-1 text-center">
            <div className="text-4xl font-bold text-primary-foreground" style={{ textShadow: '0 2px 4px rgba(0, 0, 0, 0.3)' }}>
              {user_info.data.response?.user_name}
            </div>
            <div className="text-sm text-primary-foreground/80" style={{ textShadow: '0 2px 4px rgba(0, 0, 0, 0.3)' }}>
              {user_info.data.response?.email}
            </div>
          </div>
        </div>
      </div>
      <Tabs defaultValue="lists" className="border-b">
        <TabsList className="flex">
          <TabsTrigger value="lists">Lists</TabsTrigger>
          <TabsTrigger value="likes">Liked Lists</TabsTrigger>
          <TabsTrigger value="followers">Followers</TabsTrigger>
          <TabsTrigger value="following">Following</TabsTrigger>
          <TabsTrigger value="groups">My Groups</TabsTrigger>
          <TabsTrigger value="account">Account</TabsTrigger>
        </TabsList>
        <TabsContent value="lists" className="p-6">
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Lists</div>
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
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Liked Lists</div>
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
          <div className="grid gap-4">
            <div className="text-3xl font-semibold">Groups</div>
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
