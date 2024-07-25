import { ROUTES } from "@/lib/routes";
import axios from "axios";
import {
  AnimeListResponseType,
  AnimeQueryType,
  AnimeResponseType,
  CreateLikeType,
  CreateListItemType,
  CreateListType,
  FollowListResponseType,
  FollowMutualListResponseType,
  GroupListResposeType,
  GroupQueryType,
  GroupResponseType,
  LikeListResponseType,
  LikeResponseType,
  ListItemResponseType,
  ListListResponseType,
  ListResponseType,
  ListType,
  ListTypeResponseType,
  LoginResponseType,
  MediaListResponseType,
  MediaQueryType,
  MovieListResponseType,
  MovieQueryType,
  MovieResponseType,
  RegisterResponseType,
  SongListResponseType,
  SongQueryType,
  SongResponseType,
  UserListResponseType,
  UserResponseType,
  VideoGameListResponseType,
  VideoGameQueryType,
  VideoGameResponseType,
  DeleteLikeType,
} from "../../services/api.types";

const BASE_URL = `http://127.0.0.1:5000/api`;
const axiosInstance = axios.create({ baseURL: BASE_URL });

// Auth Requests
export const loginUser = async (username: string, password: string) =>
  (
    await axiosInstance.post<LoginResponseType>(ROUTES.auth.login_user, {
      username,
      password,
    })
  ).data;

export const createUser = async (
  username: string,
  password: string,
  email: string
) =>
  (
    await axiosInstance.post<RegisterResponseType>(ROUTES.auth.register_user, {
      username,
      password,
      email,
    })
  ).data;

export const getCurrentUser = async () => {
  const token =
    typeof window !== "undefined" ? localStorage.getItem("token") : undefined;
  if (!token)
    return {
      success: false,
      response: {
        email: "",
        username: "",
        userpassword: "",
        createdat: undefined,
      },
      error: "token does not exist",
    } as UserResponseType;

  try {
    const user = await axiosInstance.get<UserResponseType>(
      ROUTES.auth.protected.get_user,
      {
        headers: {
          Authorization: "Bearer " + localStorage.getItem("token"),
        },
      }
    );
    return user.data;
  } catch (err) {
    return {
      success: false,
      response: {
        email: "",
        username: "",
        userpassword: "",
        createdat: undefined,
      },
      error: "token does not exist",
    } as UserResponseType;
  }
};

// Anime Requests
export const getAnime = async (query: AnimeQueryType) =>
  (
    await axiosInstance.get<AnimeListResponseType>(
      `${ROUTES.anime.get_anime}?${ROUTES.anime.create_query_string(query)}`
    )
  ).data;

export const getAnimeById = async (id: string) =>
  (await axiosInstance.get<AnimeResponseType>(ROUTES.anime.get_anime_by_id(id)))
    .data;

// Movies Requests
export const getMovies = async (query: MovieQueryType) =>
  (
    await axiosInstance.get<MovieListResponseType>(
      `${ROUTES.movies.get_movies}?${ROUTES.movies.create_query_string(query)}`
    )
  ).data;
export const getMovieById = async (id: string) =>
  (
    await axiosInstance.get<MovieResponseType>(
      ROUTES.movies.get_movie_by_id(id)
    )
  ).data;

// Songs Requests
export const getSongs = async (query: SongQueryType) =>
  (
    await axiosInstance.get<SongListResponseType>(
      `${ROUTES.songs.get_songs}?${ROUTES.songs.create_query_string(query)}`
    )
  ).data;
export const getSongById = async (id: string) =>
  (await axiosInstance.get<SongResponseType>(ROUTES.songs.get_songs_by_id(id)))
    .data;

// VideoGames Requests
export const getVideoGames = async (query: VideoGameQueryType) =>
  (
    await axiosInstance.get<VideoGameListResponseType>(
      `${ROUTES.games.get_games}?${ROUTES.games.create_query_string(query)}`
    )
  ).data;

export const getVideoGameById = async (id: string) =>
  (
    await axiosInstance.get<VideoGameResponseType>(
      ROUTES.games.get_game_by_id(id)
    )
  ).data;
// Media Requests
export const getMedia = async (query: MediaQueryType) =>
  (
    await axiosInstance.get<MediaListResponseType>(
      `${ROUTES.media.get_media}?${ROUTES.media.create_query_string(query)}`
    )
  ).data;

// Lists Requests
export const getAllLists = async () =>
  (await axiosInstance.get<ListListResponseType>(ROUTES.lists.get_lists)).data
    .response;

export const getTopLists = async () =>
  (await axiosInstance.get<ListListResponseType>(ROUTES.lists.get_top_lists))
    .data.response;

export const createList = async (create: CreateListType) =>
  (
    await axiosInstance.post<ListResponseType>(
      ROUTES.lists.protected.create_list(create.list_items[0].username),
      create,
      {
        headers: {
          Authorization: "Bearer " + localStorage.getItem("token"),
        },
      }
    )
  ).data;

export const createListItem = async (create: CreateListItemType) =>
  (
    await axiosInstance.post<ListItemResponseType>(
      ROUTES.listitems.protected.create_list_item(
        create.list_name,
        String(create.item_id)
      ),
      create
    )
  ).data;

// Users Requests
export const getUsersLists = async (user_name: string) =>
  (
    await axiosInstance.get<ListListResponseType>(
      ROUTES.users.get_user_lists(user_name)
    )
  ).data;

export const getUserList = async <T>(user_name: string, list_name: string) =>
  (await axiosInstance.get<T>(ROUTES.users.get_user_list(user_name, list_name)))
    .data;

export const getUserListType = async (user_name: string, list_name: string) =>
  (
    await axiosInstance.get<ListTypeResponseType>(
      ROUTES.users.get_user_list_type(user_name, list_name)
    )
  ).data;

export const getUsersLikes = async (user_name: string) =>
  (
    await axiosInstance.get<LikeListResponseType>(
      ROUTES.users.get_likes(user_name)
    )
  ).data;

export const getAllUsers = async () =>
  (await axiosInstance.get<UserListResponseType>(ROUTES.users.get_users)).data;

export const getUserByUsername = async (user_name: string) =>
  (
    await axiosInstance.get<UserResponseType>(
      ROUTES.users.get_user_by_username(user_name)
    )
  ).data;

export const deleteUser = async (user_name: string) =>
  (
    await axiosInstance.delete<UserResponseType>(
      ROUTES.users.protected.delete_user(user_name),
      {
        headers: {
          Authorization: "Bearer " + localStorage.getItem("token"),
        },
      }
    )
  ).data;

// Likes Routes
export const getAllLikes = async () => {
  (await axiosInstance.get<LikeListResponseType>(ROUTES.likes.get_likes)).data;
};

export const getUserLikes = async (user_name: string) =>
  (
    await axiosInstance.get<LikeListResponseType>(
      ROUTES.likes.get_user_likes(user_name)
    )
  ).data;

// Follows Routes

export const getUserFollowing = async (user_name: string) =>
  (
    await axiosInstance.get<FollowListResponseType>(
      ROUTES.follow.get_user_follows(user_name)
    )
  ).data;

export const getUserFollowers = async (user_name: string) =>
  (
    await axiosInstance.get<FollowMutualListResponseType>(
      ROUTES.follow.get_user_mutuals(user_name)
    )
  ).data;

// Groups Routes
export const getUserGroups = async (user_name: string) =>
  (
    await axiosInstance.get<GroupListResposeType>(
      ROUTES.groups.get_group_by_username(user_name)
    )
  ).data;

export const getGroupById = async (gid: string) =>
  (
    await axiosInstance.get<GroupResponseType>(
      ROUTES.groups.get_group_by_gid(gid)
    )
  ).data;

export const getGroupMemberLists = async (gid: string, query: GroupQueryType) =>
  (
    await axiosInstance.get<ListListResponseType>(
      `${ROUTES.groups.get_group_lists(
        gid
      )}?${ROUTES.groups.create_query_string(query)}`
    )
  ).data;

export const getRecommendedGroups = async (gid: string) =>
  (
    await axiosInstance.get<GroupListResposeType>(
      ROUTES.groups.get_group_circles(gid)
    )
  ).data;

export const getAllMediaByType = async (list_type: ListType) => {
  switch (list_type) {
    case "anime":
      return await getAnime({ page: 0, limit: 100 });
    case "movies":
      return await getMovies({ page: 0, limit: 100 });
    case "songs":
      return await getSongs({ page: 0, limit: 100 });
    case "videogames":
      return await getVideoGames({ page: 0, limit: 100 });
  }
};

export const getMediaByTypeAndId = async (list_type: ListType, id: string) => {
  switch (list_type) {
    case "anime":
      return await getAnimeById(id);
    case "movies":
      return await getMovieById(id);
    case "songs":
      return await getSongById(id);
    case "videogames":
      return await getVideoGameById(id);
  }
};

// Explore Routes
export const getRecommendedLists = async () => {
  const token = localStorage.getItem("token");
  if (!token) {
    return undefined;
  }
  const user = await getCurrentUser();
  return (
    await axiosInstance.get<ListListResponseType>(
      ROUTES.explore.protected.explore(user.response.username),
      {
        headers: {
          Authorization: "Bearer " + token,
        },
      }
    )
  ).data.response;
};

export const createLike = async (create: CreateLikeType) => {
  const token = localStorage.getItem("token");
  if (!token) {
    return undefined;
  }
  return (
    await axiosInstance.post<LikeResponseType>(
      ROUTES.likes.protected.create_like,
      create,
      {
        headers: {
          Authorization: "Bearer " + localStorage.getItem("token"),
        },
      }
    )
  ).data;
};

export const deleteLike = async (d: DeleteLikeType) => {
  const token = localStorage.getItem("token");
  if (!token) {
    return undefined;
  }
  return (
    await axiosInstance.delete<LikeResponseType>(
      ROUTES.likes.protected.delete_like(d.liking_name, d.list_name),
      {
        headers: {
          Authorization: "Bearer " + localStorage.getItem("token"),
        },
      }
    )
  ).data;
};
