import { User, UserDTO } from '../types/userTypes';

export function formatUserData(user: User): string {
    return `${user.name} (${user.email})`;
}

export function validateUserInput(input: UserDTO): boolean {
    const { name, email } = input;
    return (
        typeof name === 'string' &&
        typeof email === 'string' &&
        email.includes('@')
    );
}