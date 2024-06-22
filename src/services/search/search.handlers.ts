import { Request, Response } from 'express'
import { AnimeIdSearchType, GetAnimeType, GetTopAnimeType } from './search.schema'
import { GetAnimeByIdJikan, GetAnimeFromJikan, GetRandomAnimeFromJikan, GetTopAnimeJikan } from './search.service'

export const GetAnimeByIdHandler = async (req: Request<AnimeIdSearchType>, res: Response) => {
  const anime = await GetAnimeByIdJikan(req.params)
  return res.status(200).send(anime)
}
export const GetTopAnimeHandler = async (req: Request<{}, {}, {}, GetTopAnimeType>, res: Response) => {
  const top = await GetTopAnimeJikan(req.query)
  return res.status(200).send(top)
}
export const GetAnimeHandler = async (req: Request<{}, {}, {}, GetAnimeType>, res: Response) => {
  const anime = await GetAnimeFromJikan(req.query)
  return res.status(200).send(anime)
}
export const GetRandomAnimeHandler = async (req: Request, res: Response) => {
  const random = await GetRandomAnimeFromJikan()
  return res.status(200).send(random)
}
