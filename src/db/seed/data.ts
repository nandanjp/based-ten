import * as fs from 'fs'
import * as path from 'path'
import csv from 'csv-parser'
import {
  AggregateFileType,
  AnimeFileSchema,
  AnimeFilesType,
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

function ParseSeedData<T extends AggregateFileType>(
  seed_file_path: string,
  schema: ZodSchema<T>
): Promise<Array<T>> {
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
  anime: 'anime.txt',
  groupMembers: 'group-members.txt',
  groups: 'groups.txt',
  likes: 'likes.txt',
  listItems: 'list-items.txt',
  lists: 'lists.txt',
  movies: 'movies.txt',
  songs: 'songs.txt',
  users: 'users.txt',
  games: 'video-games.txt'
}

export const SeedAnime = async () =>
  await ParseSeedData<AnimeFilesType>(
    path.join(__dirname, 'sample-data', seed_data_paths.anime),
    AnimeFileSchema
  )

export const SeedGames = async () =>
  await ParseSeedData<VideoGamesFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.games),
    VideoGamesFileSchema
  )

export const SeedMovies = async () =>
  await ParseSeedData<MoviesFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.movies),
    MoviesFileSchema
  )

export const SeedSongs = async () =>
  await ParseSeedData<SongsFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.songs),
    SongsFileSchema
  )

export const SeedUsers = async () =>
  await ParseSeedData<UsersFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.users),
    UsersFileSchema
  )

export const SeedGroupMembers = async () =>
  await ParseSeedData<GroupMembersFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.groupMembers),
    GroupMembersFileSchema
  )

export const SeedGroups = async () =>
  await ParseSeedData<GroupsFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.groups),
    GroupsFileSchema
  )

export const SeedLikes = async () =>
  await ParseSeedData<LikesFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.likes),
    LikesFileSchema
  )

export const SeedLists = async () =>
  await ParseSeedData<ListsFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.lists),
    ListsFileSchema
  )

export const SeedListItems = async () =>
  await ParseSeedData<ListItemsFileType>(
    path.join(__dirname, 'sample-data', seed_data_paths.listItems),
    ListItemsFileSchema
  )
