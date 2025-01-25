import { Router, Application } from 'express';
import UserController from '../controllers/userController';

const router = Router();
const userController = new UserController();

export function setUserRoutes(app: Application) {
    app.use('/api/users', router);
    
    router.get('/:id', userController.getUser.bind(userController));
    router.post('/', userController.createUser.bind(userController));
    router.put('/:id', userController.updateUser.bind(userController));
}