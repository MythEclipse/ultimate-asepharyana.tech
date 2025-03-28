'use client';

import Image from 'next/image';
import React from 'react';
import { CardBody, CardContainer, CardItem } from '@/components/card/3d-card';
import { Link } from 'next-view-transitions';

interface ThreeDCardProps {
  title: string;
  description: string;
  imageUrl: string;
  linkUrl: string;
}

const ThreeDCard: React.FC<ThreeDCardProps> = ({
  title,
  description,
  imageUrl,
  linkUrl,
}) => {
  return (
    <Link href={linkUrl} scroll={true} passHref>
      <CardContainer className='inter-var cursor-pointer'>
        <CardBody className='bg-gray-50 relative group/card dark:hover:shadow-2xl shadow-blue-500/50 dark:bg-black border-blue-500 w-auto sm:w-[30rem] h-auto rounded-xl p-6 border hover:ring-4 hover:ring-gradient-to-r hover:from-blue-500 hover:to-purple-500'>
          <CardItem
            translateZ='20'
            className='text-xl font-bold text-neutral-600 dark:text-white'
          >
            {title}
          </CardItem>
          <CardItem
            as='p'
            translateZ='20'
            className='text-neutral-500 text-sm max-w-sm mt-2 dark:text-neutral-300'
          >
            {description}
          </CardItem>
          <CardItem translateZ='20' className='w-full mt-4'>
            <Image
              src={imageUrl}
              width={600}
              height={400}
              className='w-full h-60 object-cover rounded-xl'
              alt='Card Thumbnail'
              priority
              placeholder='blur'
              sizes='(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 600px'
            />
          </CardItem>
        </CardBody>
      </CardContainer>
    </Link>
  );
};

export default ThreeDCard;
