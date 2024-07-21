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
import { Button } from "../ui/button";

interface UserCardProps {
  user_email: string;
}

export function UserCard({ user_email }: UserCardProps) {
  return (
      <Card className="w-[500px]">
        <CardHeader>
          <CardTitle className="text-lg overflow-ellipsis font-normal">{user_email}</CardTitle>
        </CardHeader>

        <CardContent>
          <Link href={`/user/${user_email}`}>
            <Button variant={'default'}>
              View Profile
            </Button>
          </Link>
        </CardContent>
      </Card>
  );
}
