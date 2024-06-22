import { Router } from 'express'
import anime_router from './anime.routes'
import movie_router from './movies.routes'
import song_router from './songs.routes'
import user_router from './users.routes'
import search_router from './search.routes'
import games_router from './video-games.routes'

const router = Router()
router.use('/api/search', search_router)
// router.use('/api/anime', anime_router)
// router.use('/api/movie', movie_router)
// router.use('/api/song', song_router)
router.use('/api/user', user_router)
// router.use('/api/video-game', games_router)

export default router
