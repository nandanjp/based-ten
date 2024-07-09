import * as React from "react";

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

interface ListCardProps {
  list_name: string;
  list_author: string;
  list_type: "anime" | "movies" | "songs" | "videogames";
}

export function ListCard({ list_name, list_author, list_type }: ListCardProps) {
  return (
    <Card className="w-[400px]">
      <CardHeader>
        <CardTitle>{list_name}</CardTitle>
        <CardDescription className="italic">{list_type}</CardDescription>
      </CardHeader>
      <CardFooter className="flex justify-between">
        <p className="">Created by {list_author}</p>
      </CardFooter>
    </Card>
  );
}
