import axios from "axios";
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
  GroupsResponse,
  MediaResponse,
  LoginResponse,
  ListType,
} from "./api.types";

const BASE_URL = `http://127.0.0.1:5000/api`;
const axiosInstance = axios.create({ baseURL: BASE_URL });

export const getAllLists = async () =>
  (await axiosInstance.get<ListResponse>(`lists`)).data.response;

export const createList = async (
  user_name: string,
  list_name: string,
  list_type: ListType // right type?
) =>
  (
    await axiosInstance.post<ListResponse>("lists", {
      user_name,
      list_name,
      list_type
    })
  ).data;

export const getUsersLists =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<ListResponse>(`lists/${email}`)).data;

export const getUserList = async () =>
  (await axiosInstance.get<UserResponse[]>("users")).data;

export const getUserListItems = async ({ id }: { id: string }) =>
  (await axiosInstance.get<UserResponse>(`users/${id}`)).data;

export const getAllUsers = async () =>
  (await axiosInstance.get<UserResponse[]>("users")).data;

export const getUserByEmail =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<UserResponse>(`user/${email}`)).data;

export const deleteUser = async ({ email }: { email: string }) =>
  (await axiosInstance.delete<UserResponse>(`user/${email}`)).data;

export const getAllLikes = async () => { };

export const getUserLikes =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<LikeResponse>(`likes/${email}`)).data;

export const getUserGroups =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<GroupsResponse>(`groups/user/${email}`)).data;

export const getUserFollowing =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<FollowResponse>(`follow/${email}`)).data;

export const getUserFollowers =
  ({ email }: { email: string }) =>
    async () =>
      (await axiosInstance.get<FollowMutualResponse>(`follow/${email}/mutual`))
        .data;

export const getGroupById =
  ({ group_id }: { group_id: string }) =>
    async () =>
      (await axiosInstance.get<GroupResponse>(`groups/${group_id}`)).data;

export const getGroupMemberLists =
  ({ group_id, orderByAuthor }: { group_id: string; orderByAuthor: boolean }) =>
    async () =>
      (
        await axiosInstance.get<ListResponse>(
          `groups/${group_id}/lists?order_by_author=${orderByAuthor}`
        )
      ).data;

export const getRecommendedGroups =
  ({ group_id }: { group_id: string }) =>
    async () =>
      (await axiosInstance.get<GroupResponse>(`groups/${group_id}/circles`)).data;

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
export const getRecommendedLists = async () => {
  const token = localStorage.getItem("token");
  return (
    await axiosInstance.get<ListResponse>(`explore/user`, {
      headers: {
        Authorization: "Bearer " + token,
      },
    })
  ).data.response;
};

export const getList = (list_name: string, user_name: string) => async () => {
  const token = localStorage.getItem("token");
  return (
    await axiosInstance.get<MediaResponse>(`lists/${user_name}/${list_name}/items`, {
      headers: {
        Authorization: "Bearer " + token,
      },
    })
  ).data;
};

export const loginUser = async (user_name: string, password: string) =>
  (
    await axiosInstance.post<LoginResponse>("auth/login", {
      user_name,
      password,
    })
  ).data;

export const createUser = async (
  user_name: string,
  password: string,
  email: string
) =>
  (
    await axiosInstance.post<UserResponse>("auth/register", {
      user_name,
      password,
      email,
    })
  ).data;

export const getCurrentUser = async (): Promise<UserResponse> => {
  const token = localStorage.getItem("token");
  try {
    const result = await axiosInstance.get<UserResponse>("users/user", {
      headers: {
        Authorization: "Bearer " + token,
      },
    });
    return result.data;
  } catch {
    console.log("invalid token");
    return {
      success: false,
    };
  }
};

export const createLike = async (list_name: string, liker_name: string, liking_name: string) =>
  (await axiosInstance.post<LikeResponse>(`likes/${liker_name}/${liking_name}/${list_name}`)).data;