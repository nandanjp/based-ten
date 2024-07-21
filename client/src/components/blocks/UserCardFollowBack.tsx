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
            <div className="flex space-x-3">
              <Button variant={buttonVariant}>{buttonText}</Button>
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
