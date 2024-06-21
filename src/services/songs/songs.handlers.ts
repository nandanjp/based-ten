import { Request, Response } from 'express'
import { FilterSongType } from './songs.schema'
import prisma from '../../db/db'
import { ParamsType } from '../general.types'

export const getSongs = async (req: Request<{}, {}, {}, FilterSongType>, res: Response) => {
  const { album, creator } = req.query
  if (album && creator) {
    const songs = await prisma.songs.findMany({
      where: {
        album: {
          equals: album
        }
      }
    })
    const song_all = await prisma.media.findMany({
      where: {
        id: {
          in: songs.map((s) => s.song_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(song_all.map((s) => ({ ...s, album })))
  }
  if (album) {
    const songs = await prisma.songs.findMany({
      where: {
        album: {
          equals: album
        }
      }
    })
    const song_all = await prisma.media.findMany({
      where: {
        id: {
          in: songs.map((s) => s.song_id)
        }
      }
    })
    return res.status(200).send(song_all.map((s) => ({ ...s, album })))
  }
  if (creator) {
    const songs = await prisma.songs.findMany({})
    const song_all = await prisma.media.findMany({
      where: {
        id: {
          in: songs.map((s) => s.song_id)
        },
        creators: {
          has: creator
        }
      }
    })
    return res.status(200).send(song_all.map((s) => ({ ...s, album })))
  }
  const songs = await prisma.songs.findMany({})
  const song_all = await prisma.media.findMany({ where: { id: { in: songs.map((s) => s.song_id) } } })
  return res.status(200).send(song_all)
}

export const getSongById = async (req: Request<ParamsType>, res: Response) => {
  const { id } = req.params
  const song = await prisma.songs.findUnique({ where: { song_id: id } })
  const media = await prisma.media.findUnique({ where: { id } })
  return res.status(200).send({ ...media, album: song?.album })
}

export const createSong = async (req: Request, res: Response) => {
  res.status(201).send({ msg: 'endpoint is still not fully created... But it will create a song when done' })
}

export const deleteSongById = async (req: Request, res: Response) => {
  res.status(204).send({ msg: 'endpoint is still not fully created... But it will delete a song when done' })
}
