'use client';
import { Button } from '@/components/ui/button';
import {
  CommandDialog,
  CommandEmpty,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { Plus } from 'lucide-react';
import { useState } from 'react';
import { AddListItemProps } from './types';

export const AddListItem = (props: AddListItemProps) => {
  const { onClick, listItem, list } = props;
  const [open, setOpen] = useState<boolean>(false);
  return (
    <div className="">
      <Button
        className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-gray-600 text-white hover:bg-gray-700"
        onClick={() => {
          setOpen(true);
        }}
      >
        {listItem ? (
          <>
            <img src={listItem.media_image} className="w-12 h-12" />
            <div className="flex flex-col gap-2 items-center justify-center">
              <h1 className="font-semibold italic text-base">
                {listItem.title}
              </h1>
            </div>
          </>
        ) : (
          <>
            <Plus className="border-dashed border-2 rounded-sm" />
            <div className="flex flex-col gap-2 items-center justify-center">
              <h1 className="font-semibold italic text-base">
                Choose an item...
              </h1>
            </div>
          </>
        )}
      </Button>
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
            >
              {item.title}
            </CommandItem>
          ))}
        </CommandList>
      </CommandDialog>
    </div>
  );
};
