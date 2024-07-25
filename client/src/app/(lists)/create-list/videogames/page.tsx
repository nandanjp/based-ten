"use client";
import { createList, getVideoGameById, getVideoGames } from "@/app/actions";
import { UserContext } from "@/app/context";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { AddListItem } from "@/components/blocks/AddListItem/AddListItem";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useQuery } from "@tanstack/react-query";
import { Trash2, Upload } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useContext, useEffect, useMemo, useState } from "react";
import { ListItemType } from "../../../../../services/api.types";

const CreateListPage = ({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) => {
  const { user } = useContext(UserContext);

  const router = useRouter();
  const firstItemId = searchParams["itemId"] as string;
  const {
    data: single,
    isError: isSingleError,
    isFetching: isSingleFetching,
  } = useQuery({
    queryKey: ["get-games-by-id"],
    queryFn: async () => {
      return await getVideoGameById(firstItemId);
    },
  });
  const {
    data: all,
    isError: isAllError,
    isFetching: isAllFetching,
  } = useQuery({
    queryKey: ["get-anime"],
    queryFn: async () => {
      return await getVideoGames({ page: 0, limit: 100 });
    },
  });

  const [listName, setListName] = useState<string>("");
  const [listItems, setListItems] = useState<Array<ListItemType | undefined>>(
    Array(10)
  );

  useEffect(() => {
    if (single) {
      listItems[0] = {
        list_name: listName,
        user_name: user?.username ?? "",
        ranking_in_list: 1,
        item_id: single.response.id,
      };
      setListItems([...listItems]);
    }
  }, [single]);

  const done = useMemo(() => {
    return listItems.every((item) => !!item) && listName.trim().length > 0;
  }, [listName, listItems]);

  const onItemSelect = (index: number) => {
    return (newItem: ListItemType) => {
      const exisiting = listItems.find(
        (item) => item?.item_id == newItem.item_id
      );
      if (exisiting) {
        listItems[listItems.indexOf(exisiting)] = undefined;
      }
      listItems[index] = newItem;
      setListItems([...listItems]);
    };
  };

  const handleClickCreateList = async () => {
    if (!listItems.every((l) => l !== undefined)) return;
    const result = await createList({
      list_name: listName,
      list_type: "videogames",
      list_items: listItems,
    });
    if (result.success) {
      console.log("list created", result.response);
    } else {
      console.log("failed to create list", result.error);
    }
  };

  return (
    <div className="p-8 h-full flex justify-between w-full">
      <div className="flex flex-col">
        <Input
          className="p-0 border-none focus-visible:ring-0 focus-visible:ring-offset-0 placeholder-black text-lg font-bold placeholder-opacity-20"
          type="text"
          placeholder="Name your list..."
          onChange={(event) => {
            setListName(event.target.value);
          }}
        />
        <div className="bg-black w-16 h-px" />
        <h2 className="py-1 font-bold">
          {user ? (
            <Link
              href={`/user/${user.username}`}
              className="hover:bg-secondary p-3"
            >
              by {user.username}
            </Link>
          ) : (
            <>Unknown User</>
          )}
        </h2>
        <h3 className="font-bold italic text-sm pb-4">
          last updated {new Date().toISOString().slice(0, 10)}
        </h3>
        <div className="flex gap-2">
          <Button
            className="bg-primary hover:bg-gray-700 rounded-xl gap-2 py-5 w-fit"
            onClick={() => router.back()}
          >
            <Trash2 />
            Cancel
          </Button>
          <Button
            className={`rounded-xl gap-2 py-5 w-fit ${
              done
                ? "bg-primary hover:bg-gray-700"
                : "bg-gray-400 hover:bg-gray-400 cursor-default"
            }`}
            disabled={!done}
            onClick={handleClickCreateList}
          >
            <Upload />
            Confirm
          </Button>
        </div>
      </div>
      {isSingleError || isSingleFetching || isAllError || isAllFetching ? (
        <LoadingSpinner className="text-blue-300" />
      ) : (
        <div className="flex flex-col justify-center items-center gap-4 p-4 min-w-full">
          {listItems.map((item, index) => (
            <AddListItem
              listItem={item}
              list={all?.response}
              onClick={onItemSelect(index)}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default CreateListPage;