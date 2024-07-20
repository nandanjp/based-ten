import * as React from "react";

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

interface GroupCardProps {
  group_name: string;
  owned_by: string;
}

export function GroupCard({ group_name, owned_by }: GroupCardProps) {
  return (
    <Card className="w-[300px]">
      <CardHeader>
        <CardTitle>{group_name}</CardTitle>
      </CardHeader>
      <CardFooter className="flex justify-between">
        <p className="">Owned by {owned_by}</p>
      </CardFooter>
    </Card>
  );
}
