import { Request, Response } from 'express'
import { FilterAnimeType } from './anime.schema'
import prisma from '../../db/db'
import { ParamsType } from '../general.types'
import { equal } from 'assert'

export const getAnime = async (req: Request<{}, {}, {}, FilterAnimeType>, res: Response) => {
  const { episodes, creator } = req.query
  if (episodes && creator) {
    const anime = await prisma.anime.findMany({
      where: {
        episodes: {
          equals: parseInt(episodes)
        }
      }
    })
    const anime_all = await prisma.media.findMany({
      where: {
        id: {
          in: anime.map((a) => a.anime_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(anime_all.map((a) => ({ ...a, episodes })))
  }
  if (episodes) {
    const anime = await prisma.anime.findMany({
      where: {
        episodes: {
          equals: parseInt(episodes)
        }
      }
    })
    const anime_all = await prisma.media.findMany({
      where: {
        id: {
          in: anime.map((a) => a.anime_id)
        }
      }
    })
    return res.status(200).send(anime_all.map((a) => ({ ...a, episodes })))
  }
  if (creator) {
    const anime = await prisma.anime.findMany({})
    const anime_all = await prisma.media.findMany({
      where: {
        id: {
          in: anime.map((a) => a.anime_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(anime_all)
  }
  const anime = await prisma.anime.findMany({})
  const anime_all = await prisma.media.findMany({
    where: {
      id: {
        in: anime.map((a) => a.anime_id)
      }
    }
  })
  return res.status(200).send(anime_all)
}
export const getAnimeById = async (req: Request<ParamsType>, res: Response) => {
  const { id } = req.params
  const anime = await prisma.anime.findUnique({
    where: {
      anime_id: id
    }
  })
  const anime_all = await prisma.media.findUnique({
    where: {
      id
    }
  })
  return res.status(200).send({
    anime: {
      ...anime_all,
      episode: anime?.episodes
    }
  })
}
export const createAnime = async (req: Request, res: Response) => {
  res
    .status(201)
    .send({ msg: 'endpoint is still not fully created... But it will create an anime when done' })
}
export const deleteAnimeById = async (req: Request, res: Response) => {
  res
    .status(204)
    .send({ msg: 'endpoint is still not fully created... But it will delete an anime when done' })
}
