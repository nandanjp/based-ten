import Image from "next/image";
import { ListType } from "../../../../services/api.types";

type ViewListItemProps = {
  type: ListType;
  title: string;
  image: string;
  children: React.ReactNode;
};

const ViewListItem = ({ type, title, image, children }: ViewListItemProps) => {
  return (
    <div className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-primary text-white hover:bg-gray-700">
      <Image width={50} height={50} alt="Picture of the author" src={image} />
      <div className="flex flex-col gap-2 items-center justify-center">
        <h1 className="font-semibold italic text-base">{title}</h1>
        {children}
      </div>
    </div>
  );
};

export default ViewListItem;
