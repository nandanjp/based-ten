import { Router } from 'express'

const router = Router()

router.get('/')
router.get('/:id')
router.post('/')

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
