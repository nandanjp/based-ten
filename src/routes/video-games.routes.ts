import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { CreateVideoGameSchema, FilterVideoGameSchema } from '../services/games/games.schema'
import {
  createVideoGame,
  deleteVideoGameById,
  getVideoGameById,
  getVideoGames
} from '../services/games/games.handlers'
import { ParamsSchema } from '../services/general.types'

const router = Router()

router.get('/data')
router.get('/', validate({ query_schema: FilterVideoGameSchema }), getVideoGames)
router.get('/:id', validate({ params_schema: ParamsSchema }), getVideoGameById)
router.post('/', validate({ body_schema: CreateVideoGameSchema }), createVideoGame)
router.delete('/:id', validate({ params_schema: ParamsSchema }), deleteVideoGameById)

export default router
