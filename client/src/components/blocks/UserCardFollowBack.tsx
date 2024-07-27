import { createFollow, deleteFollow } from "@/app/actions";
import { UserContext } from "@/app/context";
import { Button, buttonVariants } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import Link from "next/link";
import { useContext, useState } from "react";

interface UserCardFollowBackProps {
  follower_email: string;
  follow_back: boolean;
}

export function UserCardFollowBack({
  follower_email,
  follow_back,
}: UserCardFollowBackProps) {
  const { user } = useContext(UserContext);
  const [followBack, setFollowBack] = useState(follow_back);
  const onFollowerButtonClick = async () => {
    if (followBack) {
      const response = await deleteFollow(follower_email);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
    } else {
      const response = await createFollow(follower_email);
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
    }
    setFollowBack(!followBack);
  };
  const buttonVariant = followBack ? "secondary" : "default";
  const buttonText = followBack ? "Following" : "Follow back";

  return (
    <Card className="min-w-96">
      <CardHeader>
        <CardTitle className="text-lg overflow-ellipsis font-normal text-center">
          {follower_email}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex gap-3 items-center justify-center">
          <Button variant={buttonVariant} onClick={onFollowerButtonClick}>
            {buttonText}
          </Button>
          <Link
            href={`/user/${follower_email}`}
            className={cn(buttonVariants({ variant: "default" }))}
          >
            View Profile
          </Link>
        </div>
      </CardContent>
    </Card>
  );
}
