import * as React from "react";

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

interface UserCardProps {
  user_email: string;
}

export function UserCard({ user_email }: UserCardProps) {
  return (
    <a href={`/user/${user_email}`} style={{ textDecoration: 'none' }}>
      <Card className="w-[500px] cursor-pointer">
        <CardHeader>
          <CardTitle className="text-lg overflow-ellipsis font-normal">{user_email}</CardTitle>
        </CardHeader>
      </Card>
    </a>
  );
}
