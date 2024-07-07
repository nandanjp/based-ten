export enum MediaType {
  SONG = 'song',
  VIDEO_GAME = 'videogame',
  ANIME = 'anime',
  MOVIE = 'movie',
}

export type Media = {
  id: number;
  created_on: string;
  media_image: string;
  title: string;
  type: MediaType;
};

export type MediaResponse = {
  error?: string;
  success: boolean;
  media: Media[];
};
