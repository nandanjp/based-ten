import { useEffect } from "react";
import {
    useUsersLists,
    useCurrentUser,
    useUserFollowing,
    useUserFollowers,
    useUserLikes,
    useUserGroups,
  } from "../../services/queries";
import { UserCardFollowBack } from "./blocks/UserCardFollowBack";

interface FollowerListProps {
    username: string;
    activeTab: string;
}

export function FollowerList({username, activeTab}: FollowerListProps) {
    const user_followers = useUserFollowers(username);
    const refetchFollowers = () => {
        user_followers.refetch();
    }
    useEffect(() => {
        if (activeTab !== 'followers') {
          // Replace this with your actual refetch function
          refetchFollowers();
        }
      }, [activeTab]);

    if (user_followers.isPending) {
        return <span>Loading....</span>;
    }

    if (user_followers.isError) {
        return <span>there was an error!</span>;
    }
    if (user_followers.isFetching) {
        return <span>data being fetched</span>;
    }
    return (
        <div className="grid gap-4">
            <div className="text-3xl font-semibold">Followers</div>
            {user_followers.data.response?.map((f) => (
              <UserCardFollowBack
                key={f.follower}
                follower_email={f.follower}
                follow_back={!!f.followsback}
              />
            ))}
        </div>
    )
}