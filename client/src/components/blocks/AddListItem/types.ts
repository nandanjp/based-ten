import {
  Anime,
  Media,
  MediaType,
  Movie,
  Song,
  VideoGame,
} from '@/../services/api.types';

export type ListItem = Anime | Movie | Song | VideoGame;

export type ListItems =
  | Array<Movie>
  | Array<Song>
  | Array<VideoGame>
  | Array<Anime>;

export type AddListItemProps = {
  listItem?: ListItem;
  list?: ListItems;
  onClick?: (newItem: ListItem) => void;
};
