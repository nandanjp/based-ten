'use client';
import { AddListItem } from '@/components/blocks/AddListItem/AddListItem';
import { ListItem, ListItems } from '@/components/blocks/AddListItem/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Trash2, Upload } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import { getAllMediaByType, getMediaByTypeAndId } from '../api/media/get-media';
import { MediaType } from '../api/media/types';
import { useRouter } from 'next/navigation';

const CreateListPage = ({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) => {
  const router = useRouter();
  const firstItemId = searchParams['itemId'] as string;
  const mediaType = searchParams['mediaType'] as MediaType;
  const [listName, setListName] = useState<string>('');
  const [listItems, setListItems] = useState<Array<ListItem | undefined>>(
    Array<ListItem>(10),
  );
  const [allItems, setAllItems] = useState<ListItems>([]);
  const done = useMemo(() => {
    return listItems.every((item) => !!item) && listName.trim().length > 0;
  }, [listName, listItems]);
  const onItemSelect = (index: number) => {
    return (newItem: ListItem) => {
      const exisiting = listItems.find((item) => item?.id == newItem.id);
      if (exisiting) {
        listItems[listItems.indexOf(exisiting)] = undefined;
      }
      listItems[index] = newItem;
      setListItems([...listItems]);
    };
  };
  useEffect(() => {
    const getFirstListItem = async () => {
      const firstItem = await getMediaByTypeAndId(mediaType, firstItemId);
      listItems[0] = firstItem;
      setListItems([...listItems]);
    };
    const getAllItems = async () => {
      const items = await getAllMediaByType(mediaType);
      setAllItems([...items]);
    };
    getFirstListItem();
    getAllItems();
  }, []);
  return (
    <div className="p-8 h-full flex justify-between">
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
        <h2 className="py-1 font-bold">by JustinLin905</h2>
        <h3 className="font-bold italic text-sm pb-4">
          last updated {new Date().toISOString().slice(0, 10)}
        </h3>
        <div className="flex gap-2">
          <Button
            className="bg-gray-600 hover:bg-gray-700 rounded-xl gap-2 py-5 w-fit"
            onClick={() => router.back()}
          >
            <Trash2 />
            Cancel
          </Button>
          <Button
            className={`rounded-xl gap-2 py-5 w-fit ${
              done
                ? 'bg-gray-600 hover:bg-gray-700'
                : 'bg-gray-400 hover:bg-gray-400 cursor-default'
            }`}
            disabled={!done}
            onClick={() => console.log('CREATE LIST')}
          >
            <Upload />
            Confirm
          </Button>
        </div>
      </div>
      <div className="flex flex-col gap-4">
        {listItems.map((item, index) => (
          <div
            key={`${index}=${item?.id}-${item?.type}`}
            className="flex gap-4 items-center justify-between"
          >
            <div className="text-4xl font-semibold text-gray-800">{`${
              index + 1
            }.`}</div>
            <AddListItem
              listItem={item}
              list={allItems}
              onClick={onItemSelect(index)}
            />
          </div>
        ))}
      </div>
    </div>
  );
};

export default CreateListPage;
