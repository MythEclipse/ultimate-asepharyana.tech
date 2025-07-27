import { prisma } from '@asepharyana/database';
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
