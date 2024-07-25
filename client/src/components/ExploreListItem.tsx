import { HeartIcon, View } from "lucide-react";

import { Button, buttonVariants } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";
import Link from "next/link";
import { createLike } from "@/app/actions";
import { UserContext } from "@/app/context";
import { useContext, useState } from "react";
import { ListType } from "../../services/api.types";

type CardProps = React.ComponentProps<typeof Card> & {
  alreadyLiked: boolean;
  type: ListType;
  title: string;
  author: string;
  numLikes: number;
};

export function ExploreListItem({
  alreadyLiked,
  type,
  className,
  title,
  author,
  numLikes,
  ...props
}: CardProps) {
  const { user } = useContext(UserContext);
  const [isLiked, setIsLike] = useState(false);
  const onLikeClick = async () => {
    setIsLike(!isLiked);

    if (user) {
      const response = await createLike({
        list_name: title,
        liking_name: author,
      });
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
      console.log("like response");
      console.log(response);
    } else {
      console.log("no user!");
    }
  };

  return (
    <Card
      className={cn(
        "w-[280px] lg:w-[380px] xl:w-[340px] 2xl:w-[420px]",
        className
      )}
      {...props}
    >
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription className="text-md">
          A{type === "anime" ? `n Anime List` : ` ${type} List`}
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div>
          <div className="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0">
            <span className="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
            <div className="space-y-1">
              <Link href={`/user/${author}`}>
                <p className="text-sm text-muted-foreground">
                  Created by {author}
                </p>
              </Link>
            </div>
          </div>
        </div>
        <div className=" flex items-center space-x-4 rounded-md border p-4">
          <Button onClick={onLikeClick} variant={"secondary"}>
            <HeartIcon
              className={cn(
                "w-4 h-4 cursor-pointer",
                isLiked || alreadyLiked ? "fill-pink-300" : "fill-none"
              )}
            />
          </Button>
          <div className="flex-1 space-y-1">
            <p className="text-sm font-medium leading-none">{numLikes} Likes</p>
          </div>
        </div>
      </CardContent>
      <CardFooter>
        <Link
          href={`/view-list/${author}/${title}`}
          className={cn(buttonVariants({ variant: "outline" }), "w-full")}
        >
          <View className="mr-2 h-4 w-4" /> View List
        </Link>
      </CardFooter>
    </Card>
  );
}
