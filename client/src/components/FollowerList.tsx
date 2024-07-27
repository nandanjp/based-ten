import { useEffect } from "react";
import { useUserFollowers } from "../../services/queries";
import { UserCardFollowBack } from "./blocks/UserCardFollowBack";

interface FollowerListProps {
  username: string;
  activeTab: string;
}

export function FollowerList({ username, activeTab }: FollowerListProps) {
  const user_followers = useUserFollowers(username);
  useEffect(() => {
    if (activeTab !== "followers") {
      user_followers.refetch();
    }
  }, [activeTab]);
  useEffect(() => {
    user_followers.refetch();
  }, [username]);

  if (user_followers.isPending) {
    return <span>Loading....</span>;
  }

  if (user_followers.isError) {
    return <span>There was an error!</span>;
  }
  if (user_followers.isFetching) {
    return <span></span>;
  }
  return (
    <div className="min-w-full flex-1 grid lg:grid-cols-3 gap-6 max-w-8xl md:grid-cols-2 grid-cols-1">
      {user_followers.data.response?.map((f) => (
        <UserCardFollowBack
          key={f.follower}
          follower_email={f.follower}
          follow_back={!!f.followsback}
        />
      ))}
    </div>
  );
}
