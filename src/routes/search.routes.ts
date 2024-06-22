import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { AnimeIdSearchSchema, GetAnimeFilterSchema, TopAnimeFilterSchema } from '../services/search/search.schema'
import { GetAnimeByIdHandler, GetAnimeHandler, GetRandomAnimeHandler, GetTopAnimeHandler } from '../services/search/search.handlers'

const router = Router()

//http://localhost:3000/api/search/anime?limit=10&page=1&status=complete&rating=pg&sort=asc&order_by=mal_id
router.get('/anime', validate({ query_schema: GetAnimeFilterSchema }), GetAnimeHandler)
router.get('/anime/top', validate({ query_schema: TopAnimeFilterSchema }), GetTopAnimeHandler)
router.get('/anime/random', GetRandomAnimeHandler)
router.get('/anime/:id', validate({ params_schema: AnimeIdSearchSchema }), GetAnimeByIdHandler)

export default router
