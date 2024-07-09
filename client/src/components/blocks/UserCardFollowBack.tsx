import * as React from "react";

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button"

interface UserCardFollowBackProps {
  follower_email: string;
  follows_back: boolean;
}

export function UserCardFollowBack({ follower_email, follows_back }: UserCardFollowBackProps) {
  const buttonVariant = follows_back ? "secondary" : "default"
  const buttonText = follows_back ? "Following" : "Follow back"
  return (
    <Card className="w-[500px]">
      <CardHeader>
        <div className="flex p-4 justify-between">
            <CardTitle className="text-lg overflow-ellipsis font-normal">{follower_email}</CardTitle>
            <Button variant={buttonVariant}>{buttonText}</Button>
        </div>
      </CardHeader>
    </Card>
  );
}
