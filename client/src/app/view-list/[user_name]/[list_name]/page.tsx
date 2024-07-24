"use client";
import { useParams } from "next/navigation";
import { useListByName } from "../../../../../services/queries";
import ViewListItem from "@/components/blocks/ViewListItem/ViewListItem";

const ViewListPage = () => {
  const { list_name, user_name } = useParams<{
    list_name: string;
    user_name: string;
  }>();

  const list = useListByName(user_name, list_name);

  if (list.isPending) {
    return <span>Loading...</span>;
  }

  if (list.isError) {
    return <span>There was an error in retrieving the list!</span>;
  }

  if (list.isFetching) {
    return <span>Fetching the list...</span>;
  }

  if (!list.data) {
    return <span>There is no list data!</span>;
  }

  return (
    <div className="p-8 h-full flex justify-between">
      <div className="flex flex-col">
        <h1 className="text-4xl font-bold text-gray-800">
          {decodeURIComponent(list_name)}
        </h1>
        <div className="bg-black w-16 h-px" />
        <h2 className="py-1 font-bold"></h2>
        <h3 className="font-bold italic text-sm pb-4">
          created by {user_name}
        </h3>
        <h3 className="font-bold italic text-sm pb-4">
          type: {list.data.response?.[0]?.listtype}
        </h3>
        <h3 className="font-bold italic text-sm pb-4">
          {list.data.response?.[0]?.likes} likes
        </h3>
        <div className="flex gap-2"></div>
      </div>
      <div className="flex flex-col gap-4">
        {list.data.response?.map((item, index) => (
          <div key={index} className="flex gap-4 items-center justify-between">
            <div className="text-4xl font-semibold text-gray-800">{`${index + 1
              }.`}</div>

            <ViewListItem
              type={list.data.response?.[0]?.listtype}
              listItem={item}
            />
          </div>
        ))}
      </div>
    </div>
  );
};

export default ViewListPage;
