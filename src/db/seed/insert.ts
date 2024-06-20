import format from 'pg-format'
import { client } from '../db'
import { SeedAnime, SeedGames, SeedMovies, SeedSongs, SeedUsers } from './data'
import { z } from 'zod'
import { QueryConfig } from 'pg'

const insertAnime = async () => {
  const seed_data = await SeedAnime()
  await insert(seed_data, (anime) => ({
    text: 'INSERT INTO Anime(title, mediaImage, numEpisodes, createdOn) VALUES($1, $2, $3, $4) RETURNING id, title, mediaImage, numEpisodes;',
    values: [anime.title, anime.image, anime.episodes, anime.created_at]
  }))
}

const insertGames = async () => {
  await insert(SeedGames, (game) => ({
    text: 'INSERT INTO VideoGames(title, mediaImage, console, createdOn) VALUES($1, $2, $3, $4) RETURNING id, title, mediaImage, console;',
    values: [game.name, game.image, game.console, game.created_at]
  }))
}

const insertMovies = async () => {
  await insert(SeedMovies, (movie) => ({
    text: 'INSERT INTO Movies(title, mediaImage, createdOn) VALUES($1, $2, $3) RETURNING id, title, mediaImage;',
    values: [movie.name, movie.image, movie.created_at]
  }))
}

const insertSongs = async () => {
  await insert(SeedSongs, (song) => ({
    text: 'INSERT INTO Songs(title, author, album, mediaImage, createdOn) VALUES($1, $2, $3, $4, $5) RETURNING id, author, album, title, mediaImage;',
    values: [song.name, song.creator, song.album, song.image, song.created_at]
  }))
}

const insertUsers = async () => {
  await insert(SeedUsers, (user) => ({
    text: 'INSERT INTO Users(email, displayName, userPassword) VALUES($1, $2, $3) RETURNING email, displayName, userPassword;',
    values: [user.email, user.display_name, user.password]
  }))
}

async function insert<T>(values: Array<T>, map_fn: (t: T) => QueryConfig) {
  try {
    await client.connect()
    for (const query of values.map(map_fn)) {
      const res = await client.query({
        text: format(query.text),
        values: query.values
      })
      console.log(res.rows)
    }
  } catch (error) {
    console.error('Error executing SQL:', error)
  } finally {
    await client.end()
  }
}

const ProcessArgSchema = z.array(z.string()).min(3)
ProcessArgSchema.parse(process.argv)

const ProvidedCorrectInsert = z.enum(['anime', 'games', 'movies', 'songs', 'users', 'all'])
ProvidedCorrectInsert.parse(process.argv[2])

declare global {
  namespace NodeJS {
    interface ProcessArgv extends z.infer<typeof ProcessArgSchema> {}
  }
}

async function main() {
  switch (process.argv[2] as z.infer<typeof ProvidedCorrectInsert>) {
    case 'anime':
      await insertAnime()
      break
    case 'games':
      await insertGames()
      break
    case 'movies':
      await insertMovies()
      break
    case 'songs':
      await insertSongs()
      break
    case 'users':
      await insertUsers()
      break
    case 'all':
      await insertAnime()
      await insertGames()
      await insertMovies()
      await insertSongs()
      await insertUsers()
      break
    default:
      console.error(
        `This should have been flagged as you did not provided a correct insert table name: ${process.argv[2]}`
      )
  }
}

main()
