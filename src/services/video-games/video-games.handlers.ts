import { Request, Response } from 'express'
import prisma from '../../db/db'
import { ParamsType } from '../general.types'
import { FilterVideoGameType } from './video-games.schema'

export const getVideoGames = async (req: Request<{}, {}, {}, FilterVideoGameType>, res: Response) => {
  const { creator, platform } = req.query
  if (creator && platform) {
    const games = await prisma.videoGames.findMany({
      where: {
        platforms: {
          has: platform
        }
      }
    })
    const games_all = await prisma.media.findMany({
      where: {
        id: {
          in: games.map((g) => g.video_game_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(games_all.map((g) => ({ ...g, platform })))
  }
  if (platform) {
    const games = await prisma.videoGames.findMany({
      where: {
        platforms: {
          has: platform
        }
      }
    })
    const games_all = await prisma.media.findMany({
      where: {
        id: {
          in: games.map((g) => g.video_game_id)
        }
      }
    })
    return res.status(200).send(games_all.map((g) => ({ ...g, platform })))
  }
  if (creator) {
    const games = await prisma.videoGames.findMany({})
    const games_all = await prisma.media.findMany({
      where: {
        id: {
          in: games.map((g) => g.video_game_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(games_all.map((g) => ({ ...g, platform })))
  }
  const games = await prisma.videoGames.findMany({})
  const games_all = await prisma.media.findMany({
    where: {
      id: {
        in: games.map((g) => g.video_game_id)
      }
    }
  })
  return res.status(200).send(games_all)
}

export const getVideoGameById = async (req: Request<ParamsType>, res: Response) => {
  const { id } = req.params
  const game = await prisma.videoGames.findUnique({
    where: {
      video_game_id: id
    }
  })
  const game_all = await prisma.media.findUnique({
    where: { id }
  })
  return res.status(200).send({ ...game_all, platform: game?.platforms })
}

export const createVideoGame = async (req: Request, res: Response) => {
  res
    .status(201)
    .send({ msg: 'endpoint is still not fully created... But it will create a video game when done' })
}

export const deleteVideoGameById = async (req: Request, res: Response) => {
  res
    .status(201)
    .send({ msg: 'endpoint is still not fully created... But it will delete a video game when done' })
}
