import Image from "next/image";
import {
  Anime,
  MediaType,
  Movie,
  Song,
  VideoGame,
} from "../../../../services/api.types";

type ViewListItemProps = {
  type: MediaType;
  listItem: VideoGame | Anime | Movie | Song;
};

const TypeSpecificFields = ({ type, listItem }: ViewListItemProps) => {
  switch (type) {
    case MediaType.ANIME:
      return (
        <span className="text-sm text-blue-300 font-bold">
          number of episodes: {(listItem as Anime).num_episodes}
        </span>
      );
    case MediaType.MOVIE:
      return (
        <span className="text-sm text-blue-300 font-bold">
          number of episodes: {(listItem as Movie).title}
        </span>
      );
    case MediaType.SONG:
      return (
        <span className="text-sm text-blue-300 font-bold flex flex-col gap-1 items-start justify-start">
          <span>author: {(listItem as Song).author}</span>
          <span>album: {(listItem as Song).album}</span>
        </span>
      );
    case MediaType.VIDEO_GAME:
      return (
        <span className="text-sm text-blue-300 font-bold">
          consoles: {(listItem as VideoGame).console}
        </span>
      );
  }
};

const ViewListItem = ({ type, listItem }: ViewListItemProps) => {
  return (
    <div className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-primary text-white hover:bg-gray-700">
      <Image
        width={50}
        height={50}
        alt="Picture of the author"
        src={listItem.media_image}
      />
      <div className="flex flex-col gap-2 items-center justify-center">
        <h1 className="font-semibold italic text-base">{listItem.title}</h1>
        <TypeSpecificFields type={type} listItem={listItem} />
      </div>
    </div>
  );
};

export default ViewListItem;
