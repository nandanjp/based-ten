import * as fs from 'fs'
import * as path from 'path'
import csv from 'csv-parser'
import {
  AggregateFileType,
  AnimeFileSchema,
  AnimeFilesType,
  FollowsFileSchema,
  FollowsFileType,
  GroupMembersFileSchema,
  GroupMembersFileType,
  GroupsFileSchema,
  GroupsFileType,
  LikesFileSchema,
  LikesFileType,
  ListItemsFileSchema,
  ListItemsFileType,
  ListsFileSchema,
  ListsFileType,
  MoviesFileSchema,
  MoviesFileType,
  SongsFileSchema,
  SongsFileType,
  UsersFileSchema,
  UsersFileType,
  VideoGamesFileSchema,
  VideoGamesFileType
} from './data-schemas'
import { ZodSchema } from 'zod'

function ParseSeedData<T extends AggregateFileType>(seed_file_path: string, schema: ZodSchema<T>): Promise<Array<T>> {
  const res: Array<T> = []
  return new Promise((resolve, reject) => {
    fs.createReadStream(seed_file_path)
      .pipe(csv())
      .on('data', (row) => {
        const parsed = schema.safeParse(row)
        if (parsed.error) return reject(parsed.error)

        res.push(parsed.data)
      })
      .on('end', () => {
        console.log('CSV file successfully processed')
        resolve(res)
      })
      .on('error', (error) => {
        console.error('Error reading the CSV file:', error)
        reject(error)
      })
  })
}

const seed_data_paths = {
  anime: 'anime.csv',
  groupMembers: 'group-members.csv',
  groups: 'groups.csv',
  likes: 'likes.csv',
  listItems: 'list-items.csv',
  lists: 'lists.csv',
  movies: 'movies.csv',
  songs: 'songs.csv',
  users: 'users.csv',
  games: 'video-games.csv',
  follows: 'follows.csv'
} as const
const get_path = (table: keyof typeof seed_data_paths) => path.join(__dirname, '..', 'sample-data', seed_data_paths[table])

export const SeedAnime = async () => await ParseSeedData<AnimeFilesType>(get_path('anime'), AnimeFileSchema)
export const SeedGames = async () => await ParseSeedData<VideoGamesFileType>(get_path('games'), VideoGamesFileSchema)
export const SeedMovies = async () => await ParseSeedData<MoviesFileType>(get_path('movies'), MoviesFileSchema)
export const SeedSongs = async () => await ParseSeedData<SongsFileType>(get_path('songs'), SongsFileSchema)
export const SeedUsers = async () => await ParseSeedData<UsersFileType>(get_path('users'), UsersFileSchema)
export const SeedGroupMembers = async () => await ParseSeedData<GroupMembersFileType>(get_path('groupMembers'), GroupMembersFileSchema)
export const SeedGroups = async () => await ParseSeedData<GroupsFileType>(get_path('groups'), GroupsFileSchema)
export const SeedLikes = async () => await ParseSeedData<LikesFileType>(get_path('likes'), LikesFileSchema)
export const SeedFollows = async () => await ParseSeedData<FollowsFileType>(get_path('follows'), FollowsFileSchema)
export const SeedLists = async () => await ParseSeedData<ListsFileType>(get_path('lists'), ListsFileSchema)
export const SeedListItems = async () => await ParseSeedData<ListItemsFileType>(get_path('listItems'), ListItemsFileSchema)
