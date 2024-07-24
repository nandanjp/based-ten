"use client";
import { Button } from "@/components/ui/button";
import {
  CommandDialog,
  CommandEmpty,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Plus } from "lucide-react";
import { useState } from "react";
import { AddListItemProps } from "./types";
import Image from "next/image";
import { WobbleCard } from "@/components/animated/WobbleCard";

export const AddListItem = (props: AddListItemProps) => {
  const { onClick, listItem, list } = props;
  const [open, setOpen] = useState<boolean>(false);
  return (
    <div
      onClick={() => {
        setOpen(true);
      }}
      className="max-w-9xl flexcol p-6"
    >
      <WobbleCard containerClassName="col-span-1 lg:col-span-2 bg-blue-900 min-h-[500px] lg:min-h-[600px] xl:min-h-[300px]">
        <div className="max-w-sm">
          <h2 className="max-w-sm md:max-w-lg  text-left text-balance text-base md:text-xl lg:text-3xl font-semibold tracking-[-0.015em] text-white">
            Signup for blazing-fast cutting-edge state of the art Gippity AI
            wrapper today!
            {listItem ? listItem.title : "click to add a list item"}
          </h2>
          <p className="mt-4 max-w-[26rem] text-left  text-base/6 text-neutral-200">
            With over 100,000 mothly active bot users, Gippity AI is the most
            popular AI platform for developers.
            {listItem
              ? listItem.title
              : "you can find the description if you add an item to the list....."}
          </p>
        </div>
        {listItem ? (
          <Image
            src={listItem.media_image}
            alt={listItem.title}
            width={500}
            height={500}
            className="absolute -right-10 md:-right-[40%] lg:-right-[20%] -bottom-10 object-contain rounded-2xl"
          />
        ) : (
          <Image
            src={"/howls-1.jpeg"}
            alt={"placeholder image"}
            width={500}
            height={500}
            className="absolute -right-10 md:-right-[40%] lg:-right-[20%] -bottom-10 object-contain rounded-2xl"
          />
        )}
        <CommandDialog open={open} onOpenChange={setOpen}>
          <CommandInput placeholder="Search" />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
            {list?.map((item) => (
              <CommandItem
                onSelect={() => {
                  onClick?.(item);
                  setOpen(false);
                }}
                key={`${item.id}-${item.type}`}
              >
                {item.title}
              </CommandItem>
            ))}
          </CommandList>
        </CommandDialog>
      </WobbleCard>
    </div>
  );
};
