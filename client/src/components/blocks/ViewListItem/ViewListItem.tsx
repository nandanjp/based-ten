import { ViewListItemProps } from "./types";

const ViewListItem = (props: ViewListItemProps) => {
  const { listItem } = props;
  return (
    <div
        className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-primary text-white hover:bg-gray-700"
      >
        <img src={listItem.media_image} className="w-12 h-12" />
        <div className="flex flex-col gap-2 items-center justify-center">
          <h1 className="font-semibold italic text-base">
            {listItem.title}
          </h1>
        </div>
      </div>
  );
};

export default ViewListItem;
