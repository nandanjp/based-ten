import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { CreateVideoGameSchema, FilterVideoGameSchema } from '../services/video-games/video-games.schema'
import {
  createVideoGame,
  deleteVideoGameById,
  getVideoGameById,
  getVideoGames
} from '../services/video-games/video-games.handlers'
import { ParamsSchema } from '../services/general.types'

const router = Router()

router.get('/', validate({ query_schema: FilterVideoGameSchema }), getVideoGames)
router.get('/:id', validate({ params_schema: ParamsSchema }), getVideoGameById)
router.post('/', validate({ body_schema: CreateVideoGameSchema }), createVideoGame)
router.delete('/:id', validate({ params_schema: ParamsSchema }), deleteVideoGameById)

export default router
