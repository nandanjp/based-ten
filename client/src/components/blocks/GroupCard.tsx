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

interface GroupCardProps {
  group_name: string;
  owned_by: string;
  group_id: number;
}

export function GroupCard({ group_name, owned_by, group_id }: GroupCardProps) {
  return (
    <Card className="w-[300px]">
      <CardHeader>
        <CardTitle>{group_name}</CardTitle>
      </CardHeader>
      <CardFooter className="flex justify-between">
        <p className="">Owned by {owned_by}</p>
      </CardFooter>
      <CardContent>
          <Link href={`/group/${group_id}`}>
            <Button variant={'default'}>
              View Group
            </Button>
          </Link>
      </CardContent>
    </Card>
  );
}
