import format from 'pg-format'
import { client } from '../db'
import {
  SeedAnime,
  SeedGames,
  SeedGroupMembers,
  SeedGroups,
  SeedLikes,
  SeedListItems,
  SeedLists,
  SeedMovies,
  SeedSongs,
  SeedUsers,
  SeedFollows
} from './data'
import { z } from 'zod'
import { QueryConfig } from 'pg'
import {
  AnimeFilesType,
  FollowsFileType,
  GroupMembersFileType,
  GroupsFileType,
  LikesFileType,
  ListItemsFileType,
  ListsFileType,
  MoviesFileType,
  SongsFileType,
  UsersFileType,
  VideoGamesFileType
} from './data-schemas'

type Tables =
  | 'anime'
  | 'games'
  | 'movies'
  | 'songs'
  | 'users'
  | 'groups'
  | 'group_members'
  | 'lists'
  | 'list_items'
  | 'likes'
  | 'follows'

const insert_anime = (anime: AnimeFilesType) => ({
  text: 'INSERT INTO Anime(title, mediaImage, numEpisodes, createdOn) VALUES($1, $2, $3, $4) RETURNING id, title, mediaImage, numEpisodes;',
  values: [anime.title, anime.image, anime.episodes, anime.released_on]
})
const insert_game = (game: VideoGamesFileType) => ({
  text: 'INSERT INTO VideoGames(title, mediaImage, console, createdOn) VALUES($1, $2, $3, $4) RETURNING id, title, mediaImage, console;',
  values: [game.title, game.image, game.platform, game.released_on]
})
const insert_movie = (movie: MoviesFileType) => ({
  text: 'INSERT INTO Movies(title, mediaImage, createdOn) VALUES($1, $2, $3) RETURNING id, title, mediaImage;',
  values: [movie.title, movie.image, movie.released_on]
})
const insert_song = (song: SongsFileType) => ({
  text: 'INSERT INTO Songs(title, author, album, mediaImage, createdOn) VALUES($1, $2, $3, $4, $5) RETURNING id, author, album, title, mediaImage;',
  values: [song.title, song.writer, song.album, song.image, song.released_in_year]
})
const insert_user = (user: UsersFileType) => ({
  text: 'INSERT INTO Users(email, displayName, userPassword) VALUES($1, $2, $3) RETURNING email, displayName, userPassword;',
  values: [user.user_email, user.user_name, user.password]
})
const insert_group = (group: GroupsFileType) => ({
  text: 'INSERT INTO Groups(groupName,ownedBy) VALUES($1, $2) RETURNING gid, groupName, ownedBy;',
  values: [group.name, group.user_email]
})
const insert_group_member = (group_member: GroupMembersFileType) => ({
  text: 'INSERT INTO GroupMembers(gid, email) VALUES($1, $2) RETURNING gid, email;',
  values: [group_member.group_id, group_member.member_email]
})
const insert_like = (like: LikesFileType) => ({
  text: 'INSERT INTO Likes(likerEmail, likingEmail, listName) VALUES($1, $2, $3) RETURNING likerEmail, likingEmail, listName;',
  values: [like.liker_email, like.liking_email, like.list_name]
})
const insert_list = (list: ListsFileType) => ({
  text: 'INSERT INTO Lists(email, listName, listType) VALUES($1, $2, $3) RETURNING email, listName, listType;',
  values: [list.user_email, list.list_name, list.list_type]
})
const insert_list_item = (insert_list_item: ListItemsFileType) => ({
  text: 'INSERT INTO ListItems(email, listName, itemID, rankingInList) VALUES($1, $2, $3, $4) RETURNING email, listName, itemID, rankingInList;',
  values: [
    insert_list_item.user_email,
    insert_list_item.list_name,
    insert_list_item.item_id,
    insert_list_item.rank_in_list
  ]
})
const insert_follow = (follow: FollowsFileType) => ({
  text: 'INSERT INTO Follows(followerEmail, followingEmail) VALUES($1, $2) RETURNING followerEmail, followingEmail;',
  values: [follow.follower_email, follow.following_email]
})

const insertToTable = async (table: Tables) => {
  switch (table) {
    case 'anime': {
      const seed_data = await SeedAnime()
      await insert(seed_data, insert_anime)
      break
    }
    case 'follows': {
      const seed_data = await SeedFollows()
      await insert(seed_data, insert_follow)
      break
    }
    case 'games': {
      const seed_data = await SeedGames()
      await insert(seed_data, insert_game)
      break
    }
    case 'group_members': {
      const seed_data = await SeedGroupMembers()
      await insert(seed_data, insert_group_member)
      break
    }
    case 'groups': {
      const seed_data = await SeedGroups()
      await insert(seed_data, insert_group)
      break
    }
    case 'likes': {
      const seed_data = await SeedLikes()
      await insert(seed_data, insert_like)
      break
    }
    case 'list_items': {
      const seed_data = await SeedListItems()
      await insert(seed_data, insert_list_item)
      break
    }
    case 'lists': {
      const seed_data = await SeedLists()
      await insert(seed_data, insert_list)
      break
    }
    case 'movies': {
      const seed_data = await SeedMovies()
      await insert(seed_data, insert_movie)
      break
    }
    case 'songs': {
      const seed_data = await SeedSongs()
      await insert(seed_data, insert_song)
      break
    }
    case 'users': {
      const seed_data = await SeedUsers()
      await insert(seed_data, insert_user)
      break
    }
  }
}

async function insert<T>(values: Array<T>, map_fn: (t: T) => QueryConfig) {
  try {
    for (const query of values.map(map_fn)) {
      const res = await client.query({
        text: format(query.text),
        values: query.values
      })
      console.log(res.rows)
    }
  } catch (error) {
    console.error('Error executing SQL:', error)
  }
}

const ProcessArgSchema = z.array(z.string()).min(3)
ProcessArgSchema.parse(process.argv)

const ProvidedCorrectInsert = z.enum([
  'anime',
  'games',
  'movies',
  'songs',
  'users',
  'groups',
  'group-members',
  'likes',
  'lists',
  'list-items',
  'follows',
  'all'
])
ProvidedCorrectInsert.parse(process.argv[2])

declare global {
  namespace NodeJS {
    interface ProcessArgv extends z.infer<typeof ProcessArgSchema> {}
  }
}

async function main() {
  await client.connect()
  switch (process.argv[2] as z.infer<typeof ProvidedCorrectInsert>) {
    case 'anime':
      await insertToTable('anime')
      break
    case 'games':
      await insertToTable('games')
      break
    case 'movies':
      await insertToTable('movies')
      break
    case 'songs':
      await insertToTable('songs')
      break
    case 'users':
      await insertToTable('users')
      break
    case 'groups':
      await insertToTable('groups')
      break
    case 'group-members':
      await insertToTable('group_members')
      break
    case 'likes':
      await insertToTable('likes')
      break
    case 'lists':
      await insertToTable('lists')
      break
    case 'list-items':
      await insertToTable('list_items')
      break
    case 'follows':
      await insertToTable('follows')
      break
    case 'all':
      await insertToTable('anime')
      await insertToTable('games')
      await insertToTable('movies')
      await insertToTable('songs')
      await insertToTable('users')
      await insertToTable('groups')
      await insertToTable('group_members')
      await insertToTable('lists')
      await insertToTable('likes')
      await insertToTable('list_items')
      await insertToTable('follows')
  }
  client.end()
}

main()
