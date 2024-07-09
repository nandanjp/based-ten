import axios from 'axios';
import {
  CreateUserType,
  ListResponse,
  UserResponse,
  FollowResponse,
  FollowMutualResponse,
  LikeResponse,
  Anime,
  Media,
  MediaType,
  Movie,
  Song,
  VideoGame,
} from './api.types';

const BASE_URL = `http://127.0.0.1:5000/api`;
const axiosInstance = axios.create({ baseURL: BASE_URL });

export const getAllAnime = async () => {};

export const getAnimeById = async () => {};

export const createAnimeById = async () => {};

export const updateAnimeById = async () => [];

export const deleteAnimeById = async () => {
  '/anime';
};

export const getAllMovies = async () => {};

export const getMovieById = async () => {};

export const createMovie = async () => {};

export const updateMovie = async () => {};

export const deleteMovie = async () => {};

export const getAllSongs = async () => {};

export const getSongById = async () => {};

export const createSong = async () => {};

export const updateSong = async () => {};

export const deleteSong = async () => {};

export const getAllGames = async () => {};

export const getGameById = async () => {};

export const createGame = async () => {};

export const updateGame = async () => {};

export const deleteGame = async () => {};

export const getAllLists = async () => {};

export const createList = async () => {};

export const getUsersLists =
  ({ email }: { email: string }) =>
  async () =>
    (await axiosInstance.get<ListResponse>(`lists/${email}`)).data;

export const getUserList = async () =>
  (await axiosInstance.get<UserResponse[]>('users')).data;

export const getUserListItems = async ({ id }: { id: string }) =>
  (await axiosInstance.get<UserResponse>(`users/${id}`)).data;

export const updateUserList = async () => {};

export const deleteUserList = async () => {};

export const getUserListItem = async () => {};

export const createUserListItem = async () => {};

export const updateUserListItem = async () => {};

export const deleteUserListItem = async () => {};

export const getAllUsers = async () =>
  (await axiosInstance.get<UserResponse[]>('users')).data;

export const getUserByEmail =
  ({ email }: { email: string }) =>
  async () =>
    (await axiosInstance.get<UserResponse>(`users/${email}`)).data;

export const createUser = async (user: CreateUserType) =>
  (await axiosInstance.post<UserResponse[]>(`users`, user)).data;

export const updateUser = async () => {};

export const deleteUser = async ({ email }: { email: string }) =>
  (await axiosInstance.delete<UserResponse>(`users/${email}`)).data;

export const getAllLikes = async () => {};

export const getUserLikes =
  ({ email }: { email: string }) =>
  async () =>
    (await axiosInstance.get<LikeResponse>(`likes/${email}`)).data;

export const createLike = async () => [];

export const deleteLike = async () => {};

export const getAllFollows = async () => {};

export const createFollows = async () => {};

export const getUserFollowing =
  ({ email }: { email: string }) =>
  async () =>
    (await axiosInstance.get<FollowResponse>(`follow/${email}`)).data;

export const getUserFollowers =
  ({ email }: { email: string }) =>
  async () =>
    (await axiosInstance.get<FollowMutualResponse>(`follow/mutual/${email}`))
      .data;

export const deleteUserFollower = async () => {};

export const getAllGroups = async () => {};

export const getGroupById = async () => {};

export const createGroup = async () => {};

export const deleteGroup = async () => {};

export const getAllMedia = async () => {
  const rawResult = await axiosInstance.get(`media`);
  return rawResult.data.media as Media[];
};

export const getAllMediaByType = (mediaType: MediaType) => async () => {
  const rawResult = await axiosInstance.get(`${mediaType}`);
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

export const getMediaByTypeAndId =
  (mediaType: MediaType, id: string) => async () => {
    const rawResult = await axiosInstance.get(`${mediaType}/${id}`);
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
