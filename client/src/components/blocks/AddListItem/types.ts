import {
  Anime,
  Media,
  MediaType,
  Movie,
  Song,
  VideoGame,
} from '@/app/api/media/types';

export type ListItem =
  | { mediaType: MediaType.ANIME; item: Anime }
  | { mediaType: MediaType.MOVIE; item: Movie }
  | { mediaType: MediaType.SONG; item: Song }
  | { mediaType: MediaType.VIDEO_GAME; item: VideoGame };

export type AddListItemProps = {
  listItem?: ListItem;
  onClick?: () => void;
};
