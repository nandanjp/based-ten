import { DiameterIcon, View } from "lucide-react";
import { joinGroup, unjoinGroup } from "@/app/actions";
import { UserContext } from "@/app/context";
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
import { useRouter } from "next/navigation";
import { useContext, useState, useEffect } from "react";

type CardProps = React.ComponentProps<typeof Card> & {
  alreadyFollows: boolean;
  className?: string;
  groupname: string;
  owner: string;
  numMembers: number;
  gid: number;
};

export function ExploreGroupItem({
  alreadyFollows,
  className,
  owner,
  numMembers,
  groupname,
  gid,
}: CardProps) {
  const router = useRouter();
  const { user } = useContext(UserContext);
  const [isFollowing, setIsFollowing] = useState(alreadyFollows);
  const [numberOfMembers, setNumberOfMembers] = useState(numMembers);

  useEffect(() => {
    setIsFollowing(alreadyFollows);
  }, [alreadyFollows]);

  const onLikeClick = async () => {
    if (!user) {
      router.push("/login");
      return;
    }
    const response = !isFollowing
      ? await joinGroup(gid)
      : await unjoinGroup(gid);
    if (response?.error) {
      const message = `An error has occurred: ${response.error}`;
      throw new Error(message);
    }
    setNumberOfMembers(numberOfMembers + (isFollowing ? -1 : 1));
    setIsFollowing(!isFollowing);
  };


  return (
    <Card
      className={cn(
        "w-[280px] lg:w-[380px] xl:w-[340px] 2xl:w-[420px]",
        className
      )}
    >
      <CardHeader>
        <CardTitle>{groupname}</CardTitle>
        <CardDescription className="text-md">A Group</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div>
          <div className="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0">
            <span className="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
            <div className="space-y-1">
              <Link href={`/user/${owner}`}>
                <p className="text-sm text-muted-foreground">
                  Created by {owner}
                </p>
              </Link>
            </div>
          </div>
        </div>
        <div className=" flex items-center space-x-4 rounded-md border p-4">
          <Button onClick={onLikeClick} variant={"secondary"}>
            <DiameterIcon
              className={cn(
                "w-4 h-4 cursor-pointer",
                isFollowing ? "fill-green-300" : "fill-none"
              )}
            />
          </Button>
          <div className="flex-1 space-y-1">
            <p className="text-sm font-medium leading-none">
              {numberOfMembers} Number of Members
            </p>
          </div>
        </div>
      </CardContent>
      <CardFooter>
        <Link
          href={`/group/${gid}`}
          className={cn(buttonVariants({ variant: "outline" }), "w-full")}
        >
          <View className="mr-2 h-4 w-4" /> View Group
        </Link>
      </CardFooter>
    </Card>
  );
}
