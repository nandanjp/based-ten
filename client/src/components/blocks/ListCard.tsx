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
import { CardSample } from "../animated/GoodCard";

interface ListCardProps {
  list_name: string;
  list_author: string;
  list_type?: "anime" | "movies" | "songs" | "videogames";
}

export function ListCard({ list_name, list_author, list_type }: ListCardProps) {
  return (
    <CardSample
      title={`${list_name}: Another ${list_type} List`}
      description={`Created by ${list_author}`}
    >
      <Link href={`/view-list/${list_author}/${list_name}`}>
        <Button variant={"default"}>View List</Button>
      </Link>
    </CardSample>
  );
}
