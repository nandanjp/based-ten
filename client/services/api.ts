import axios from 'axios';
import {
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
  GroupResponse,
  MediaResponse,
  LoginResponse,
} from './api.types';

const BASE_URL = `http://localhost:5000/api`;
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

export const getAllLists = async () =>
  (await axiosInstance.get<ListResponse>(`lists`)).data.response;

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

export const getGroupById =
  ({ group_id }: { group_id: string }) =>
  async () =>
    (await axiosInstance.get<GroupResponse>(`groups/${group_id}`)).data;

export const getGroupMemberLists =
  ({ group_id, orderByAuthor }: { group_id: string; orderByAuthor: boolean }) =>
  async () =>
    (
      await axiosInstance.get<ListResponse>(
        `groups/${group_id}/lists?order_by_author=${orderByAuthor}`,
      )
    ).data;

export const getRecommendedGroups =
  ({ group_id }: { group_id: string }) =>
  async () =>
    (await axiosInstance.get<GroupResponse>(`groups/${group_id}/circles`)).data;

export const createGroup = async () => {};

export const deleteGroup = async () => {};

export const getAllMedia = async () => {
  const rawResult = await axiosInstance.get(`media`);
  return rawResult.data.response as Media[];
};

export const getAllMediaByType = (mediaType: MediaType) => async () => {
  const rawResult = await axiosInstance.get(`${mediaType}`);
  let items: Anime[] | Movie[] | Song[] | VideoGame[];
  switch (mediaType) {
    case MediaType.ANIME:
      items = rawResult.data.response as Anime[];
      break;
    case MediaType.MOVIE:
      items = rawResult.data.response as Movie[];
      break;
    case MediaType.SONG:
      items = rawResult.data.response as Song[];
      break;
    case MediaType.VIDEO_GAME:
      items = rawResult.data.response as VideoGame[];
  }
  return items;
};

export const getMediaByTypeAndId =
  (mediaType: MediaType, id: string) => async () => {
    const rawResult = await axiosInstance.get(`${mediaType}/${id}`);
    let item: Song | Anime | Movie | VideoGame;
    switch (mediaType) {
      case MediaType.ANIME:
        item = rawResult.data.response as Anime;
        break;
      case MediaType.MOVIE:
        item = rawResult.data.response as Movie;
        break;
      case MediaType.SONG:
        item = rawResult.data.response as Song;
        break;
      case MediaType.VIDEO_GAME:
        item = rawResult.data.response as VideoGame;
    }
    return item;
  };

export const getRecommendedLists = (userId: string) => async () =>
  (await axiosInstance.get<ListResponse>(`explore/${userId}`)).data.response;

export const getList = (list_name: string) => async () =>
  (await axiosInstance.get<MediaResponse>(`lists/view/${list_name}`)).data;

export const getUser = async (user_name: string, password: string) =>
  (
    await axiosInstance.post<LoginResponse>('auth/login', {
      user_name,
      password,
    })
  ).data;

export const createUser = async (
  user_name: string,
  password: string,
  email: string,
) =>
  (
    await axiosInstance.post<UserResponse>('auth/register', {
      user_name,
      password,
      email,
    })
  ).data;
