import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { CreateAnimeSchema, FilterAnimeSchema } from '../services/anime/anime.schema'
import { createAnime, deleteAnimeById, getAnime, getAnimeById } from '../services/anime/anime.handlers'
import { ParamsSchema } from '../services/general.types'

const router = Router()

router.get('/', validate({ query_schema: FilterAnimeSchema }), getAnime)
router.get('/:id', validate({ params_schema: ParamsSchema }), getAnimeById)
router.post('/', validate({ body_schema: CreateAnimeSchema }), createAnime)
router.delete('/:id', validate({ params_schema: ParamsSchema }), deleteAnimeById)

export default router
