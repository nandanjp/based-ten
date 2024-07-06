import { Request, Response } from 'express'
import { ParamsType } from '../general.types'
import { prisma } from '../../db/db'
import { FilterMovieType } from './movies.schema'

export const getMovies = async (req: Request<{}, {}, {}, FilterMovieType>, res: Response) => {
  const { creator } = req.query
  if (creator) {
    const movies = await prisma.movies.findMany({})
    const movie_all = await prisma.media.findMany({
      where: {
        id: {
          in: movies.map((m) => m.movie_id)
        },
        creators: {
          equals: [creator]
        }
      }
    })
    return res.status(200).send(movie_all)
  }
  const movie = await prisma.movies.findMany({})
  const movie_all = await prisma.media.findMany({
    where: {
      id: {
        in: movie.map((m) => m.movie_id)
      }
    }
  })
  return res.status(200).send(movie_all)
}

export const getMovieById = async (req: Request<ParamsType>, res: Response) => {
  const { id } = req.params
  const movie = await prisma.movies.findUnique({
    where: {
      movie_id: id
    }
  })
  const movie_all = await prisma.media.findUnique({
    where: {
      id
    }
  })
  return res.status(200).send({
    movie: movie_all
  })
}

export const createMovie = async (req: Request, res: Response) => {
  res.status(201).send({ msg: 'endpoint is still not fully created... But it will create a movie when done' })
}

export const deleteMovieById = async (req: Request, res: Response) => {
  res.status(204).send({ msg: 'endpoint is still not fully created... But it will delete a movie when done' })
}
