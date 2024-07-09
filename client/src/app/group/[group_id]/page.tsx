"use client";

import { ListCard } from "@/components/blocks/ListCard";
import { useParams } from "next/navigation";
import { useGroupById } from "../../../../services/queries";

const GroupPage = () => {
  const { group_id } = useParams<{ group_id: string }>();
  const group_info = useGroupById({ group_id });

  if (group_info.isPending) {
    return <span>Loading....</span>;
  }

  if (group_info.isError) {
    return <span>there was an error!</span>;
  }

  console.log(group_info.data)

  return (
    <div className="w-full xl mx-auto">
      <div className="bg-primary p-6">
        <div className="flex items-center pt-12 pl-6 pb-6">
          <div className="grid gap-1">
            <div className="text-4xl font-bold text-primary-foreground">
              {group_info.data.group?.group_name}
            </div>
            <div className="text-sm text-primary-foreground/80">
              Owner: {group_info.data.group?.owned_by}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
export default GroupPage;
