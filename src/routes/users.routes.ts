import { Router } from 'express'
import { createUser, getUserById, getUsers } from '../services/users/users.handlers'
import { validate } from '../middleware/validate-schema'
import { ParamsSchema } from '../services/general.types'
import { CreateUserSchema } from '../services/users/users.schema'

const router = Router()

router.get('/', getUsers)
router.get('/:id', validate({ params_schema: ParamsSchema }), getUserById)
router.post('/', validate({ body_schema: CreateUserSchema }), createUser)

export default router

/*
Still to implement:
/api/users/:id/groups
/api/users/:id/lists
/api/users/:id/lists/:id
POST:
/api/users/:id/groups
/api/users/:id/lists
*/
