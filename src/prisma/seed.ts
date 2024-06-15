import { SeedAnime } from './anime'
import { SeedGames } from './games'
import { SeedSongs } from './song'
import { SeedMovies } from './movies'
import { SeedUsers } from './users'

import { PrismaClient } from '@prisma/client'

const prisma = new PrismaClient()

async function main() {
  for (const anime of SeedAnime) {
    const created = await prisma.media.create({
      data: {
        title: anime.name,
        image: anime.image,
        creators: [anime.studio],
        created_at: anime.created_at
      }
    })
    await prisma.anime.create({
      data: {
        anime_id: created.id,
        episodes: anime.episodes
      }
    })
  }

  for (const game of SeedGames) {
    const created = await prisma.media.create({
      data: {
        title: game.name,
        image: game.image,
        creators: [game.console],
        created_at: game.created_at
      }
    })
    await prisma.videoGames.create({
      data: {
        video_game_id: created.id,
        platforms: [game.console]
      }
    })
  }

  for (const movie of SeedMovies) {
    const created = await prisma.media.create({
      data: {
        title: movie.name,
        image: movie.image,
        created_at: movie.created_at
      }
    })
    await prisma.movies.create({
      data: {
        movie_id: created.id
      }
    })
  }

  for (const song of SeedSongs) {
    const created = await prisma.media.create({
      data: {
        title: song.name,
        image: song.image,
        creators: [song.creator],
        created_at: song.created_at
      }
    })
    await prisma.songs.create({
      data: {
        song_id: created.id,
        album: song.album
      }
    })
  }

  for (const user of SeedUsers) {
    await prisma.user.create({
      data: {
        name: user.name,
        display_name: user.display_name,
        email: user.email,
        password: user.password
      }
    })
  }
}

main()
  .then(async () => {
    await prisma.$disconnect()
  })
  .catch(async (e) => {
    console.error(e)
    await prisma.$disconnect()
    process.exit(1)
  })
  .finally(async () => {
    await prisma.$disconnect()
  })
