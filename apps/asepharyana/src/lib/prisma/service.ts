import { prisma } from '../db';
export { prisma };




export const updateUserImage = async (id: string, image: string) => {
  try {
    const user = await prisma.user.update({
      where: {
        id,
      },
      data: {
        image,
      },
    });
    return user;
  } catch (error) {
    console.error('Error updating user image:', error);
    throw new Error('Failed to update user image');
  }
};

export const getUserById = async (id: string) => {
  try {

    const user = await prisma.user.findUnique({
      where: {
        id,
      },
    });
    return user;
  } catch (error) {
    console.error('Error fetching user by ID:', error);
    throw new Error('Failed to fetch user by ID');
  }
};

export const upsertGoogleUser = async (email: string, name: string | null, image: string | null) => {
  try {
    const user = await prisma.user.upsert({
      where: { email: email },
      update: {
        name: name,
        image: image,
      },
      create: {
        email: email,
        name: name,
        image: image,
        role: 'member', // Default role for new Google users
      },
    });
    return user;
  } catch (error) {
    console.error('Error upserting Google user:', error);
    throw new Error('Failed to upsert Google user');
  }
};
