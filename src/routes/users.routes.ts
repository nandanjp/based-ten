import { Router } from 'express'
import { createUser, getUserById, getUsers } from '../services/users/users.handlers'
import { validate } from '../middleware/validate-schema'
import { ParamsSchema } from '../services/general.types'
import { CreateUserSchema } from '../services/users/users.schema'

const router = Router()

router.get('/', getUsers)
router.get('/:id', validate({ params_schema: ParamsSchema }), getUserById)
router.get('/:id/groups', validate({ params_schema: ParamsSchema }))
router.get('/:id/lists', validate({ params_schema: ParamsSchema }))
router.get('/:id/lists/:id', validate({ params_schema: ParamsSchema }))
router.post('/', validate({ body_schema: CreateUserSchema }), createUser)
router.post('/:id/groups', validate({ body_schema: CreateUserSchema }), createUser)
router.post('/:id/lists', validate({ body_schema: CreateUserSchema }), createUser)

export default router
