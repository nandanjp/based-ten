import { Request, Response } from 'express'
import prisma from '../../db/db'
import { ParamsType } from '../general.types'

export const getUsers = async (req: Request, res: Response) => {
  const users = await prisma.user.findMany({})
  return res.status(200).send(users)
}

export const getUserById = async (req: Request<ParamsType>, res: Response) => {
  const { id } = req.params
  const user = await prisma.user.findUnique({
    where: {
      user_id: id
    }
  })
  return res.status(200).send(user)
}

export const createUser = async (req: Request, res: Response) => {
  res.status(201).send({ msg: 'endpoint is still not fully created... But it will create a user when done' })
}
