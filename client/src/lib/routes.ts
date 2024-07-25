import {
  AnimeQueryType,
  MovieQueryType,
  SongQueryType,
  VideoGameQueryType,
  MediaQueryType,
  GroupQueryType,
} from "../../services/api.types";

export const ROUTES = {
  auth: {
    register_user: "/auth/register",
    login_user: "/auth/login",
    protected: {
      logout_user: "/auth/logout",
      get_user: "/auth/user",
    },
  },
  anime: {
    get_anime: "/anime",
    get_anime_by_id: (id: string) => `/anime/${id}`,
    create_query_string: ({
      page,
      limit,
      num_episodes,
      sort_by,
      title,
    }: AnimeQueryType) =>
      `page=${page}&limit=${limit}${
        num_episodes ? "&num_episodes=" + num_episodes : ""
      }${sort_by ? "&sort_by=" + sort_by : ""}${
        title ? "&title=" + title : ""
      }`,
  },
  movies: {
    get_movies: "/movies",
    get_movie_by_id: (id: string) => `/movies/${id}`,
    create_query_string: ({ page, limit, sort_by, title }: MovieQueryType) =>
      `page=${page}&limit=${limit}${sort_by ? "&sort_by=" + sort_by : ""}${
        title ? "&title=" + title : ""
      }`,
  },
  songs: {
    get_songs: "/songs",
    get_songs_by_id: (id: string) => `/songs/${id}`,
    create_query_string: ({
      page,
      limit,
      sort_by,
      title,
      author,
    }: SongQueryType) =>
      `page=${page}&limit=${limit}${sort_by ? "&sort_by=" + sort_by : ""}${
        title ? "&title=" + title : ""
      }${author ? "&author=" + author : ""}`,
  },
  games: {
    get_games: "/videogames",
    get_game_by_id: (id: string) => `/videogames/${id}`,
    create_query_string: ({
      page,
      limit,
      console,
      sort_by,
      title,
    }: VideoGameQueryType) =>
      `page=${page}&limit=${limit}${sort_by ? "&sort_by=" + sort_by : ""}${
        title ? "&title=" + title : ""
      }${console ? "&console=" + console : ""}`,
  },
  media: {
    get_media: "/media",
    create_query_string: ({
      page,
      limit,
      sort_by,
      title,
      type,
    }: MediaQueryType) =>
      `page=${page}&limit=${limit}${sort_by ? "&sort_by=" + sort_by : ""}${
        title ? "&title=" + title : ""
      }${type ? "&type=" + type : ""}`,
  },
  users: {
    get_users: "/users",
    create_user: "/users",
    get_user_by_username: (user_name: string) => `/users/${user_name}/info`,
    get_user_lists: (user_name: string) => `/users/${user_name}`,
    get_user_list: (user_name: string, list_name: string) =>
      `/users/${user_name}/${list_name}`,
    get_user_list_type: (user_name: string, list_name: string) =>
      `/users/${user_name}/${list_name}/type`,
    protected: {
      get_user: (user_name: string) => `/users/${user_name}/me`,
      update_user: (user_name: string) => `/users/${user_name}/me`,
      delete_user: (user_name: string) => `/users/${user_name}/me`,
    },
  },
  lists: {
    get_lists: "/lists",
    get_top_lists: "/lists/top",
    protected: {
      create_list: (user_name: string) => `/lists/${user_name}`,
      update_list: (user_name: string, list_name: string) =>
        `/lists/${user_name}/${list_name}`,
      delete_list: (user_name: string, list_name: string) =>
        `/lists/${user_name}/${list_name}`,
    },
  },
  likes: {
    get_likes: "/likes",
    get_user_likes: (user_name: string) => `/likes/${user_name}`,
    protected: {
      create_like: "/likes",
      delete_like: (liking: string, list_name: string) =>
        `/likes/${liking}/${list_name}`,
    },
  },
  follow: {
    get_follows: "/follow",
    get_user_follows: (user_name: string) => `/follow/${user_name}/following`,
    get_user_mutuals: (user_name: string) => `/follow/${user_name}/mutual`,
    protected: {
      create_follow: "/follow",
      delete_follow: (following: string) => `/follow/${following}`,
    },
  },
  groups: {
    get_groups: "/groups",
    get_group_by_username: (user_name: string) => `/groups/${user_name}/groups`,
    get_group_by_gid: (gid: string) => `/groups/${gid}/group`,
    get_group_members: (gid: string) => `/groups/${gid}/members`,
    get_group_lists: (gid: string) => `/groups/${gid}/lists`,
    get_group_circles: (gid: string) => `/groups/${gid}/circles`,
    protected: {
      create_group: (user_name: string) => `/groups/${user_name}/me`,
      delete_group: (user_name: string, group_name: string) =>
        `/groups/${user_name}/me/${group_name}`,
    },
    create_query_string: ({ order_by_author }: GroupQueryType) =>
      `order_by_author=${order_by_author ? order_by_author : false}`,
  },
  explore: {
    protected: {
      explore: (user_name: string) => `/explore/${user_name}`,
    },
  },
} as const;
