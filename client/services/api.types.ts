import { z } from "zod";

export const listType = z.enum(["anime", "songs", "movies", "videogames"]);
export type ListType = z.infer<typeof listType>;

type ApiResponseType<T> = {
  success: boolean;
  response: T;
  error?: string;
};

type ListApiResponseType<T> = ApiResponseType<Array<T>>;

// Auth Request/Response Objects
const registerSchema = z.object({
  email: z.string().email(),
  user_name: z.string(),
  password: z.string(),
  created_at: z.date(),
});

const loginSchema = z.object({
  token: z.string(),
});

export type RegisterResponseType = ApiResponseType<
  z.infer<typeof registerSchema>
>;
export type LoginResponseType = ApiResponseType<z.infer<typeof loginSchema>>;

// Anime Request/Response Objects
const animeSchema = z.object({
  id: z.number(),
  title: z.string(),
  mediaimage: z.string(),
  numepisodes: z.number(),
  createdon: z.date().optional(),
});

const animeQuerySchema = z.object({
  title: z.string().optional(),
  num_episodes: z.number().optional(),
  page: z.number().positive().default(0),
  limit: z.number().positive().default(100),
  sort_by: z.enum(["numepisodes", "title"]).optional(),
});

export type AnimeResponseType = ApiResponseType<z.infer<typeof animeSchema>>;
export type AnimeListResponseType = ListApiResponseType<
  z.infer<typeof animeSchema>
>;
export type AnimeQueryType = z.infer<typeof animeQuerySchema>;

// Game Request/Response Objects
const gameSchema = z.object({
  id: z.number(),
  title: z.string(),
  mediaimage: z.string(),
  console: z.string(),
  createdon: z.date().optional(),
});

const gameQuerySchema = z.object({
  title: z.string().optional(),
  console: z.string().optional(),
  page: z.number().positive().default(0),
  limit: z.number().positive().default(100),
  sort_by: z.enum(["title", "console"]).optional(),
});

export type VideoGameResponseType = ApiResponseType<z.infer<typeof gameSchema>>;
export type VideoGameListResponseType = ListApiResponseType<
  z.infer<typeof gameSchema>
>;
export type VideoGameQueryType = z.infer<typeof gameQuerySchema>;

// Movie Request/Response Objects
const movieSchema = z.object({
  id: z.number(),
  title: z.string(),
  mediaimage: z.string(),
  createdon: z.date().optional(),
});

const movieQuerySchema = z.object({
  title: z.string().optional(),
  page: z.number().positive().default(0),
  limit: z.number().positive().default(100),
  sort_by: z.enum(["title"]).optional(),
});

export type MovieResponseType = ApiResponseType<z.infer<typeof movieSchema>>;
export type MovieListResponseType = ListApiResponseType<
  z.infer<typeof movieSchema>
>;
export type MovieQueryType = z.infer<typeof movieQuerySchema>;

// Song Request/Response Objects
const songSchema = z.object({
  id: z.number(),
  title: z.string(),
  author: z.string(),
  album: z.string(),
  mediaimage: z.string(),
  createdon: z.date().optional(),
});

const songQuerySchema = z.object({
  title: z.string().optional(),
  author: z.string().optional(),
  page: z.number().positive().default(0),
  limit: z.number().positive().default(100),
  sort_by: z.enum(["author", "title"]).optional(),
});

export type SongResponseType = ApiResponseType<z.infer<typeof songSchema>>;
export type SongListResponseType = ListApiResponseType<
  z.infer<typeof songSchema>
>;
export type SongQueryType = z.infer<typeof songQuerySchema>;

// Media Request/Response Objects
const mediaSchema = z.object({
  id: z.number().optional(),
  title: z.string().optional(),
  mediaimage: z.string().optional(),
  createdon: z.date().optional(),
  type: listType,
});

const mediaQuerySchema = z.object({
  title: z.string().optional(),
  type: listType.optional(),
  page: z.number().positive().default(0),
  limit: z.number().positive().default(100),
  sort_by: z.enum(["title", "type"]).optional(),
});

export type MediaResponseType = ApiResponseType<z.infer<typeof mediaSchema>>;
export type MediaListResponseType = ListApiResponseType<
  z.infer<typeof mediaSchema>
>;
export type MediaType = z.infer<typeof mediaSchema>;
export type MediaQueryType = z.infer<typeof mediaQuerySchema>;

// User Request/Response Objects
const userSchema = z.object({
  email: z.string(),
  username: z.string(),
  userpassword: z.string(),
  createdat: z.date().optional(),
});
export type UserType = z.infer<typeof userSchema>;
export type UserResponseType = ApiResponseType<UserType>;
export type UserListResponseType = ListApiResponseType<UserType>;

// List Request/Response Objects
const listWithLikesSchema = z.object({
  username: z.string(),
  listname: z.string(),
  list_type: listType,
  likes: z.number().optional(),
});
const listWithItemsSchema = z.object({
  username: z.string(),
  listname: z.string(),
  rankinginlist: z.number().positive(),
  itemid: z.number(),
  title: z.string(),
  media_image: z.string(),
  createdon: z.date().optional(),
  likes: z.number().optional(),
  listtype: listType,
});
const listTypeSchema = z.object({
  listtype: listType,
});
const animeListSchema = listWithItemsSchema.and(
  z.object({
    numepisodes: z.number().positive(),
  })
);
const movieListSchema = listWithItemsSchema;
const gameListSchema = listWithItemsSchema.and(
  z.object({
    console: z.string(),
  })
);
const songListSchema = listWithItemsSchema.and(
  z.object({
    author: z.string(),
    album: z.string(),
  })
);
const listItemSchema = z.object({
  user_name: z.string(),
  list_name: z.string(),
  ranking_in_list: z.number(),
  item_id: z.number(),
});
const createListSchema = z.object({
  list_name: z.string(),
  list_type: listType,
  list_items: z.array(listItemSchema),
});

export type ListResponseType = ApiResponseType<
  z.infer<typeof listWithLikesSchema>
>;
export type ListTypeResponseType = ApiResponseType<
  z.infer<typeof listTypeSchema>
>;
export type ListListResponseType = ListApiResponseType<
  z.infer<typeof listWithLikesSchema>
>;
export type ListWithItemsResponseType = ListApiResponseType<
  z.infer<typeof listWithItemsSchema>
>;
export type ListAnimeResponseType = ListApiResponseType<
  z.infer<typeof animeListSchema>
>;
export type ListMovieResponseType = ListApiResponseType<
  z.infer<typeof movieListSchema>
>;
export type ListGameResponseType = ListApiResponseType<
  z.infer<typeof gameListSchema>
>;
export type ListSongResponseType = ListApiResponseType<
  z.infer<typeof songListSchema>
>;
export type CreateListType = z.infer<typeof createListSchema>;
export type ListItemType = z.infer<typeof listItemSchema>;

// Follow Request/Response Objects
const followMutualSchema = z.object({
  follower: z.string(),
  followsback: z.boolean().optional(),
});
const followSchema = z.object({
  follower: z.string(),
  following: z.string(),
});
const followQuerySchema = z.object({
  following: z.string(),
});

export type FollowResponseType = ApiResponseType<z.infer<typeof followSchema>>;
export type FollowListResponseType = ListApiResponseType<
  z.infer<typeof followSchema>
>;
export type FollowQueryType = z.infer<typeof followQuerySchema>;
export type FollowMutualListResponseType = ListApiResponseType<
  z.infer<typeof followMutualSchema>
>;

// Groups Request/Response Objects
const groupSchema = z.object({
  gid: z.number(),
  groupname: z.string(),
  ownedby: z.string(),
});
const groupQuerySchema = z.object({
  order_by_author: z.boolean().optional(),
});
const groupRecursiveSchema = z.object({
  gid: z.number().optional(),
  groupname: z.string().optional(),
  owndby: z.string().optional(),
});
const groupMemberSchema = z.object({
  username: z.string(),
});
const createGroupSchema = z.object({
  gid: z.number(),
  group_name: z.string(),
});

export type GroupResponseType = ApiResponseType<z.infer<typeof groupSchema>>;
export type GroupListResposeType = ListApiResponseType<
  z.infer<typeof groupSchema>
>;
export type GroupQueryType = z.infer<typeof groupQuerySchema>;
export type CreateGroupType = z.infer<typeof createGroupSchema>;
export type GroupRecursiveResponseType = ListApiResponseType<
  z.infer<typeof groupRecursiveSchema>
>;
export type GroupMembersResponseType = ApiResponseType<
  z.infer<typeof groupMemberSchema>
>;

// Likes Request/Response Objects
const likeSchema = z.object({
  likername: z.string(),
  likingname: z.string(),
  listname: z.string(),
});
const likeQuerySchema = z.object({
  liking_name: z.string(),
});
const createLikeSchema = z.object({
  liking_name: z.string(),
  list_name: z.string(),
});

export type LikeResponseType = ApiResponseType<z.infer<typeof likeSchema>>;
export type LikeListResponseType = ListApiResponseType<
  z.infer<typeof likeSchema>
>;
export type LikeQueryType = z.infer<typeof likeQuerySchema>;
export type CreateLikeType = z.infer<typeof createLikeSchema>;
