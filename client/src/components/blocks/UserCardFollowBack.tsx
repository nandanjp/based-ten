import * as React from "react";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Link from "next/link";
import { Button } from "@/components/ui/button"
import { useContext, useState } from "react";
import { UserContext } from "@/app/context";
import { createFollow, deleteFollow } from "@/app/actions";

interface UserCardFollowBackProps {
  follower_email: string;
  follow_back: boolean;
}

export function UserCardFollowBack({ follower_email, follow_back}: UserCardFollowBackProps) {
  const { user } = useContext(UserContext);
  const [followBack, setFollowBack] = useState(follow_back);
  const onFollowerButtonClick = async () => {
    if (followBack) {
      const response = await deleteFollow(follower_email);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
      console.log("unfollow response");
      console.log(response);
    } else {
      const response = await createFollow(follower_email);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
      console.log("follow response");
      console.log(response);
    }
    setFollowBack(!followBack);
  };
  const buttonVariant = followBack ? "secondary" : "default"
  const buttonText = followBack ? "Following" : "Follow back"

  return (
    <Card className="w-[500px]">
      <CardHeader>
        <div className="flex p-4 justify-between">
            <CardTitle className="text-lg overflow-ellipsis font-normal">{follower_email}</CardTitle>
            <div className="flex space-x-3">
              <Button variant={buttonVariant} onClick={onFollowerButtonClick}>
                {buttonText}
              </Button>
              <Link href={`/user/${follower_email}`}>
                <Button variant={'default'}>
                  View Profile
                </Button>
              </Link>
            </div>
        </div>
      </CardHeader>
    </Card>
  );
}
