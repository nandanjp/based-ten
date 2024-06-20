// This file contains types for the text file data. It is just to help with creating typescript arrays of each object
import { z } from 'zod'

export const AnimeFileSchema = z.object({
  anime_id: z.number().positive(),
  title: z.string(),
  image: z.string().url(),
  episodes: z.number().positive(),
  released_on: z.string().date()
})

export const GroupMembersFileSchema = z.object({
  member_email: z.string().email(),
  group_id: z.number().positive()
})

export const GroupsFileSchema = z.object({
  group_id: z.number().positive(),
  name: z.string(),
  user_email: z.string().email()
})

export const LikesFileSchema = z.object({
  liker_email: z.string().email(),
  liking_email: z.string().email(),
  list_name: z.string()
})

export const ListItemsFileSchema = z.object({
  user_email: z.string().email(),
  list_name: z.string(),
  rank_in_list: z.number().positive().min(1).max(10)
})

export const ListsFileSchema = z.object({
  user_email: z.string().email(),
  list_name: z.string(),
  list_type: z.enum(['anime', 'movies', 'songs', 'videogames'])
})

export const MoviesFileSchema = z.object({
  movie_id: z.number().positive(),
  title: z.string(),
  image: z.string().url(),
  released_on: z.string().date()
})

export const SongsFileSchema = z.object({
  song_id: z.number().positive(),
  title: z.string(),
  writer: z.string(),
  album: z.string(),
  image: z.string().url(),
  released_in_year: z.string().date()
})

export const UsersFileSchema = z.object({
  user_email: z.string().email(),
  user_name: z.string(),
  password: z.string().min(6).toLowerCase()
})

export const VideoGamesFileSchema = z.object({
  game_id: z.number().positive(),
  title: z.string(),
  image: z.string().url(),
  released_on: z.string().date(),
  platform: z.enum([
    'PlayStation 3',
    'Xbox',
    'Nintendo Switch',
    'PlayStation',
    'Xbox',
    'PlayStation 4',
    'PlayStation 5'
  ])
})

export type AnimeFilesType = z.infer<typeof AnimeFileSchema>
export type GroupMembersFileType = z.infer<typeof GroupMembersFileSchema>
export type GroupsFileType = z.infer<typeof GroupsFileSchema>
export type LikesFileType = z.infer<typeof LikesFileSchema>
export type ListItemsFileType = z.infer<typeof ListItemsFileSchema>
export type ListsFileType = z.infer<typeof ListsFileSchema>
export type MoviesFileType = z.infer<typeof MoviesFileSchema>
export type SongsFileType = z.infer<typeof SongsFileSchema>
export type UsersFileType = z.infer<typeof UsersFileSchema>
export type VideoGamesFileType = z.infer<typeof VideoGamesFileSchema>

export type AggregateFileType =
  | AnimeFilesType
  | GroupMembersFileType
  | GroupsFileType
  | LikesFileType
  | ListItemsFileType
  | ListsFileType
  | MoviesFileType
  | SongsFileType
  | UsersFileType
  | VideoGamesFileType
