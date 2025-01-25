import { Request, Response } from 'express';
import { UserService } from '../services/userService';
import { UserDTO } from '../types/userTypes';

class UserController {
    private userService = new UserService();

    async getUser(req: Request, res: Response) {
        const userId = parseInt(req.params.id, 10);
        try {
            const user = await this.userService.findUser(userId);
            if (user) {
                res.json(user);
            } else {
                res.status(404).send('User not found');
            }
        } catch {
            res.status(500).send('Internal server error');
        }
    }

    async createUser(req: Request, res: Response) {
        const userData: UserDTO = req.body;
        const user = { id: '', ...userData, password: 'default' };
        try {
            await this.userService.saveUser(user);
            res.status(201).json(user);
        } catch  {
            res.status(500).send('Internal server error');
        }
    }

    async updateUser(req: Request, res: Response) {
        const userId = parseInt(req.params.id, 10);
        const updatedData: UserDTO = req.body;
        try {
            await this.userService.updateUser(userId, updatedData);
            res.json({ id: userId, ...updatedData });
        } catch {
            res.status(500).send('Internal server error');
        }
    }
}

export default UserController;