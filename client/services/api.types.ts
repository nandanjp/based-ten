export const listTypes = ["anime", "movies", "songs", "videogames"] as const;
export type ListType = (typeof listTypes)[number];

export interface AnimeResponse {
  success: boolean;
  response?: {
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
  response?: {
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
  response?: {
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
  response?: {
    id: number;
    title: string;
    media_image: string;
    created_on: string;
  };
  error?: string;
}

export interface UserResponse {
  success: boolean;
  response?: {
    email: string;
    user_name: string;
    password: string;
    created_at: string;
  };
  error?: string;
}

export interface ListResponse {
  success: boolean;
  response?: {
    user_name: string;
    list_name: string;
    list_type: ListType;
    likes: number;
  }[];
  error?: string;
}

export interface ListItemResponse {
  success: boolean;
  response?: {
    user_name: string;
    list_name: string;
    ranking_in_list: number;
    item_id: number;
  };
  error?: string;
}

export interface ListItemsResponse {
  success: boolean;
  response?: {
    user_name: string;
    list_name: string;
    ranking_in_list: number;
    item_id: number;
  }[];
  error?: string;
}

export interface LikeResponse {
  success: boolean;
  response?: {
    liker_name: string;
    liking_name: string;
    list_name: string;
  }[];
  error?: string;
}

export interface GroupResponse {
  success: boolean;
  response?: {
    gid: number;
    group_name: string;
    owned_by: string;
  };
  error?: string;
}

export interface GroupsResponse {
  success: boolean;
  response?: {
    gid: number;
    group_name: string;
    owned_by: string;
  }[];
  error?: string;
}

export interface FollowResponse {
  success: boolean;
  response?: {
    follower: string;
    following: string;
  }[];
}

export interface FollowMutualResponse {
  success: boolean;
  response?: {
    follower: string;
    follows_back: boolean;
  }[];
}

export enum MediaType {
  SONG = "songs",
  VIDEO_GAME = "videogames",
  ANIME = "anime",
  MOVIE = "movies",
}

export type Media = {
  id: number;
  created_on: string;
  media_image: string;
  title: string;
  listtype: MediaType;
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
  album: string;
};

type BaseResponse = {
  error?: string;
  success: boolean;
};

export type MediaResponse = BaseResponse & {
  response: ((VideoGame | Anime | Movie | Song) & { likes: number })[];
};

export type LoginResponse = {
  success: boolean;
  token: string;
};
