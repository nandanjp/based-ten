export enum MediaType {
  SONG = 'songs',
  VIDEO_GAME = 'videogames',
  ANIME = 'anime',
  MOVIE = 'movies',
}

export type Media = {
  id: number;
  created_on: string;
  media_image: string;
  title: string;
  type: MediaType;
};

export type VideoGame = Media & {
  console: string;
};

export type Anime = Media & {
  num_episodes: number;
};

export type Movie = Media;

export type Song = Media & {
  author: string;
};

type BaseResponse = {
  error?: string;
  success: boolean;
};

export type MediaResponse = BaseResponse & {
  items: Media[] | VideoGame[] | Anime[] | Movie[] | Song[];
};
