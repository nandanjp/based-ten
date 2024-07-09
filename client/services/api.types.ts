import { z } from "zod";

type ListType = "anime" | "movies" | "songs" | "videogames";
export const create_anime = z.object({
  id: z.number().positive(),
  title: z.string(),
  media_image: z.string(),
  num_episodes: z.number().positive(),
  created_on: z.string().date(),
});

export const create_movies = z.object({});

export const create_game = z.object({
  id: z.number().positive(),
  title: z.string(),
  media_image: z.string(),
  console: z.string(),
  created_on: z.string().date(),
});

export const create_song = z.object({
  id: z.number().positive(),
  title: z.string(),
  author: z.string(),
  media_image: z.string(),
  created_on: z.string().date(),
});

export const create_user = z.object({
  email: z.string().email(),
  display_name: z.string(),
  password: z.string(),
});

export const create_list = z.object({
  email: z.string().email(),
  list_name: z.string(),
  list_type: z.enum(["anime", "movies", "songs", "videogames"]),
});

export const create_list_item = z.object({
  email: z.string().email(),
  list_name: z.string(),
  ranking_in_list: z.number().min(1).max(10),
  item_id: z.number().positive(),
});

export const create_group = z.object({
  gid: z.number().positive(),
  group_name: z.string(),
  owned_by: z.string(),
});

export const create_follow = z.object({
  follower_email: z.string().email(),
  following_email: z.string().email(),
});

export const create_like = z.object({
  liker_email: z.string().email(),
  liking_email: z.string().email(),
  list_name: z.string(),
});

export type CreateAnimeType = z.infer<typeof create_anime>;
export type CreateMovieType = z.infer<typeof create_movies>;
export type CreateSongType = z.infer<typeof create_song>;
export type CreateVideoGameType = z.infer<typeof create_game>;
export type CreateUserType = z.infer<typeof create_user>;
export type CreateListType = z.infer<typeof create_list>;
export type CreateListItemType = z.infer<typeof create_list_item>;
export type CreateGroupType = z.infer<typeof create_group>;
export type CreateFollowType = z.infer<typeof create_follow>;
export type CreateLikeType = z.infer<typeof create_like>;

export interface AnimeResponse {
  success: boolean;
  anime?: {
    id: number;
    title: string;
    media_image: string;
    num_epiosdes: number;
    created_on: string;
  };
  error?: string;
}

export interface GameResponse {
  success: boolean;
  game?: {
    id: number;
    title: string;
    media_image: string;
    console: string;
    created_on: string;
  };
  error?: string;
}

export interface SongResponse {
  success: boolean;
  song?: {
    id: number;
    title: string;
    media_image: string;
    author: string;
    created_on: string;
  };
  error?: string;
}

export interface MovieResponse {
  success: boolean;
  movie?: {
    id: number;
    title: string;
    media_image: string;
    created_on: string;
  };
  error?: string;
}

export interface MediaResponse {
  success: boolean;
  media?: {
    id: number;
    title: string;
    media_image: string;
    created_on: string;
    media_type: ListType;
  };
  error?: string;
}

export interface UserResponse {
  success: boolean;
  user?: {
    email: string;
    display_name: string;
    password: string;
    created_at: string;
  };
  error?: string;
}

export interface ListResponse {
  success: boolean;
  lists?: {
    user_name: string;
    list_name: string;
    list_type: ListType;
  }[];
  error?: string;
}

export interface ListItemResponse {
  success: boolean;
  list_item?: {
    user_name: string;
    list_name: string;
    ranking_in_list: number;
    item_id: number;
  };
  error?: string;
}

export interface LikeResponse {
  success: boolean;
  likes?: {
    liker_name: string;
    liking_name: string;
    list_name: string;
  }[];
  error?: string;
}

export interface GroupResponse {
  success: boolean;
  groups?: {
    gid: number;
    group_name: string;
    owned_by: string;
  };
  error?: string;
}

export interface FollowResponse {
  success: boolean;
  follows?: {
    follower: string;
    following: string;
  }[];
}

export interface FollowMutualResponse {
  success: boolean;
  followmutuals?: {
    follower: string;
    follows_back: boolean;
  }[];
}
