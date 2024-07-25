"use client";

import { GroupCard } from "@/components/blocks/GroupCard";
import { ListCard } from "@/components/blocks/ListCard";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import GradientHeader from "@/components/ui/gradient-header";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@radix-ui/react-collapsible";
import { ChevronsUpDown } from "lucide-react";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";
import {
  useGroupById,
  useGroupMemberLists,
  useRecommendedGroups,
} from "../../../../services/queries";

const GroupPage = () => {
  const { group_id } = useParams<{ group_id: string }>();
  const group_info = useGroupById(group_id);
  const [orderByAuthor, setOrdering] = useState(true);
  const group_member_lists = useGroupMemberLists({
    gid: group_id,
    order_by_author: orderByAuthor,
  });
  const recommended_groups = useRecommendedGroups(group_id);
  useEffect(() => {
    group_member_lists.refetch()
  }, [orderByAuthor]);

  const handleChangeListOrdering = (value: string) => {
    console.log("handled")
    setOrdering(value == "usernames");
  };

  if (group_info.isPending || group_member_lists.isPending) {
    return <span>Loading....</span>;
  }

  if (group_info.isError || group_member_lists.isError) {
    return <span>there was an error!</span>;
  }

  console.log(group_member_lists.data);

  return (
    <div className="min-w-full min-h-full p-4">
      <GradientHeader
        title={group_info.data.response?.groupname}
        subtitle={group_info.data.response?.ownedby}
      />
      <div className="p-12">
        <div className="flex justify-between py-6">
          <div className="text-3xl font-semibold pb-6">Member lists</div>
          <Select
            defaultValue="usernames"
            onValueChange={handleChangeListOrdering}
          >
            <SelectTrigger className="w-[300px]">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="usernames">
                Sort by author usernames
              </SelectItem>
              <SelectItem value="types">Sort by list type</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="grid lg:grid-cols-3 md:grid-cols-2 gap-4">
          {group_member_lists.data.response?.map((l) => (
            <ListCard
              key={l.username.concat(l.listname)}
              list_author={l.username!}
              list_name={l.listname!}
              list_type={l.listtype!}
            />
          ))}
        </div>
      </div>

      <div className="p-12">
        <Collapsible>
          <CollapsibleTrigger className="flex gap-2">
            <div className="flex justify-between py-6">
              <div className="text-3xl font-semibold pb-6 mr-4">
                Recommended Groups
              </div>
              <ChevronsUpDown />
            </div>
          </CollapsibleTrigger>
          <CollapsibleContent className="m-4">
            <Carousel>
              <CarouselContent className="-ml-1">
                {Array.isArray(recommended_groups.data?.response) &&
                  recommended_groups.data?.response.map((g) => (
                    <CarouselItem
                      key={g.gid}
                      className="lg:basis-1/4 md:basis-1/3 sm:basis-1/2"
                    >
                      <GroupCard
                        key={g.groupname}
                        owned_by={g.ownedby!}
                        group_name={g.groupname!}
                        group_id={g.gid!}
                      />
                    </CarouselItem>
                  ))}
              </CarouselContent>
              <CarouselPrevious />
              <CarouselNext />
            </Carousel>
          </CollapsibleContent>
        </Collapsible>
      </div>
    </div>
  );
};
export default GroupPage;
