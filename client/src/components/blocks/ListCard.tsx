import Link from "next/link";
import { CardSample } from "../animated/GoodCard";
import { Button } from "../ui/button";

interface ListCardProps {
  list_name: string;
  list_author: string;
  list_type?: "anime" | "movies" | "songs" | "videogames";
}

export function ListCard({ list_name, list_author, list_type }: ListCardProps) {
  return (
    <CardSample
      title={`${list_name}${list_type ? `: Another ${list_type} List` : ""}`}
      description={`Created by ${list_author}`}
      classname="min-w-96 min-h-96"
    >
      <Link href={`/view-list/${list_author}/${list_name}`}>
        <Button variant={"default"}>View List</Button>
      </Link>
    </CardSample>
  );
}
