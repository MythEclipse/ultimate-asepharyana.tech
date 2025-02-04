import { encode } from 'blurhash';

const getImageData = (image: HTMLImageElement, width: number, height: number) => {
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  if (!ctx) return null;

  ctx.drawImage(image, 0, 0, width, height);
  return ctx.getImageData(0, 0, width, height);
};

export const encodeImageToBlurhash = async (imageUrl: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const image = new Image();
    // Gantilah URL dengan CORS Proxy
    image.src = `/api/imageproxy?url=${encodeURIComponent(imageUrl)}`;

    image.onload = () => {
      const imageData = getImageData(image, 32, 32);
      if (!imageData) return reject('Gagal mendapatkan imageData');

      const blurHash = encode(imageData.data, imageData.width, imageData.height, 4, 4);
      resolve(blurHash);
    };

    image.onerror = () => reject('Gagal memuat gambar');
  });
};
