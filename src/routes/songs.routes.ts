import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { CreateSongsSchema, FilterSongsSchema } from '../services/songs/songs.schema'
import { createSong, deleteSongById, getSongById, getSongs } from '../services/songs/songs.handlers'
import { ParamsSchema } from '../services/general.types'

const router = Router()

router.get('/', validate({ query_schema: FilterSongsSchema }), getSongs)
router.get('/:id', validate({ params_schema: ParamsSchema }), getSongById)
router.post('/', validate({ body_schema: CreateSongsSchema }), createSong)
router.delete('/:id', validate({ params_schema: ParamsSchema }), deleteSongById)

export default router
