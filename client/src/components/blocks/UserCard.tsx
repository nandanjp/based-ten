import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import Link from "next/link";
import { Button, buttonVariants } from "../ui/button";
import { cn } from "@/lib/utils";

interface UserCardProps {
  user_email: string;
}

export function UserCard({ user_email }: UserCardProps) {
  return (
    <Card className="min-w-64">
      <CardHeader>
        <div className="flex p-4 justify-between">
          <CardTitle className="text-lg overflow-ellipsis font-normal">
            {user_email}
          </CardTitle>
          <Link
            href={`/user/${user_email}`}
            className={cn(buttonVariants({ variant: "default" }))}
          >
            View Profile
          </Link>
        </div>
      </CardHeader>
    </Card>
  );
}
