'use client';
import { useState } from 'react';
import { AddListItemProps } from './types';

export const ViewListItem = (props: AddListItemProps) => {
  const { listItem } = props;
  return (
    <div className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-primary text-white">
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
        <div className="flex flex-col gap-2 items-center justify-center">
          <h1 className="font-semibold italic text-base">
            No item found
          </h1>
        </div>
      )}
    </div>
  );
};

