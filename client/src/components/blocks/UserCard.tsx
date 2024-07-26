import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import Link from "next/link";
import { Button } from "../ui/button";

interface UserCardProps {
  user_email: string;
}

export function UserCard({ user_email }: UserCardProps) {
  return (
    <Card className="w-[500px]">
      <CardHeader>
        <div className="flex p-4 justify-between">
          <CardTitle className="text-lg overflow-ellipsis font-normal">
            {user_email}
          </CardTitle>
          <div className="flex space-x-3">
            <Link href={`/user/${user_email}`}>
              <Button variant={"default"}>View Profile</Button>
            </Link>
          </div>
        </div>
      </CardHeader>
    </Card>
  );
}
