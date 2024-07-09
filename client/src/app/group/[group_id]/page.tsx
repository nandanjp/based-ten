"use client";

import { ListCard } from "@/components/blocks/ListCard";
import { useParams } from "next/navigation";

const GroupPage = () => {
  const { group_id } = useParams<{ group_id: string }>();

  return (
    <div className="w-full xl mx-auto">
      <div className="bg-primary p-6">
        <div className="flex items-center pt-12 pl-6 pb-6">
          <div className="grid gap-1">
            <div className="text-4xl font-bold text-primary-foreground">
              TEST
            </div>
            <div className="text-sm text-primary-foreground/80">
              test 2
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
export default GroupPage;
