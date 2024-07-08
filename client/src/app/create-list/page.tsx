'use client';
import { AddListItem } from '@/components/blocks/AddListItem/AddListItem';
import { ListItem } from '@/components/blocks/AddListItem/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { useEffect, useState } from 'react';
import { getMediaByTypeAndId } from '../api/media/get-media';
import { MediaType } from '../api/media/types';

const CreateListPage = ({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) => {
  const firstItemId = searchParams['itemId'] as string;
  const mediaType = searchParams['mediaType'] as MediaType;
  const [listItems, setListItems] = useState<Array<ListItem | undefined>>(
    Array<ListItem>(10),
  );
  useEffect(() => {
    const getFirstListItem = async () => {
      const firstItem = await getMediaByTypeAndId(mediaType, firstItemId);
      listItems[0] = {
        item: firstItem,
        mediaType,
      } as ListItem;
      setListItems([...listItems]);
    };
    getFirstListItem();
  }, []);
  return (
    <div className="p-8 h-full flex justify-between">
      <div className="flex flex-col">
        <Input
          className="p-0 border-none focus-visible:ring-0 focus-visible:ring-offset-0 placeholder-black text-lg font-bold placeholder-opacity-20"
          type="text"
          placeholder="Name your list..."
        />
        <div className="bg-black w-16 h-px" />
        <h2 className="py-1 font-bold">by JustinLin905</h2>
        <h3 className="font-bold italic text-sm pb-4">
          last updated {new Date().toISOString().slice(0, 10)}
        </h3>
        <Button className="bg-gray-600 hover:bg-gray-700 rounded-xl gap-2 py-5">
          <Trash2 />
          Delete this List
        </Button>
      </div>
      <div className="flex flex-col gap-4">
        <AddListItem listItem={listItems[0]} />
        <AddListItem />
        <AddListItem />
        <AddListItem />
        <AddListItem />
        <AddListItem />
        <AddListItem />
        <AddListItem />
        <AddListItem />
      </div>
    </div>
  );
};

export default CreateListPage;
