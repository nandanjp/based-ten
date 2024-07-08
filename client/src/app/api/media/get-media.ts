import axios from 'axios';
import {
  Anime,
  Media,
  MediaResponse,
  MediaType,
  Movie,
  Song,
  VideoGame,
} from './types';

const BASE_URL = 'http://127.0.0.1:5000';

export const getAllMedia = async () => {
  const rawResult = await axios.get(`${BASE_URL}/api/media`);
  return rawResult.data.media as Media[];
};

export const getAllMediaByType = async (mediaType: MediaType) => {
  const rawResult = await axios.get(`${BASE_URL}/api/${mediaType}`);
  let items: Anime[] | Movie[] | Song[] | VideoGame[];
  switch (mediaType) {
    case MediaType.ANIME:
      items = rawResult.data.anime as Anime[];
      break;
    case MediaType.MOVIE:
      items = rawResult.data.movies as Movie[];
      break;
    case MediaType.SONG:
      items = rawResult.data.songs as Song[];
      break;
    case MediaType.VIDEO_GAME:
      items = rawResult.data.games as VideoGame[];
  }
  return items;
};

export const getMediaByTypeAndId = async (mediaType: MediaType, id: string) => {
  const rawResult = await axios.get(`${BASE_URL}/api/${mediaType}/${id}`);
  let item: Song | Anime | Movie | VideoGame;
  switch (mediaType) {
    case MediaType.ANIME:
      item = rawResult.data.anime as Anime;
      break;
    case MediaType.MOVIE:
      item = rawResult.data.movie as Movie;
      break;
    case MediaType.SONG:
      item = rawResult.data.song as Song;
      break;
    case MediaType.VIDEO_GAME:
      item = rawResult.data.game as VideoGame;
  }
  return item;
};
