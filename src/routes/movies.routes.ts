import { Router } from 'express'
import { validate } from '../middleware/validate-schema'
import { CreateMoviesSchema, FilterMoviesSchema } from '../services/movies/movies.schema'
import { createMovie, deleteMovieById, getMovieById, getMovies } from '../services/movies/movie.handlers'
import { ParamsSchema } from '../services/general.types'

const router = Router()

router.get('/') //get all movies
router.get('/:id') //get movie
router.post('/') //create movie
router.patch('/:id') //update movie
router.delete('/:id') //delete movie
router.get('/query/:name') //search for movie from external API

router.get('/data')
router.get('/', validate({ query_schema: FilterMoviesSchema }), getMovies)
router.get('/:id', validate({ params_schema: ParamsSchema }), getMovieById)
router.post('/', validate({ body_schema: CreateMoviesSchema }), createMovie)
router.delete('/:id', validate({ params_schema: ParamsSchema }), deleteMovieById)

export default router
