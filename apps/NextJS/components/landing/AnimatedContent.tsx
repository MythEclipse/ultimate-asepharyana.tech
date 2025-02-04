'use client';
import React from 'react';
import Image from 'next/image';
import Instagram from '@/components/logo/Instagram';
import Facebook from '@/components/logo/Facebook';
import Linkedln from '@/components/logo/LinkedIn';
import Discord from '@/components/logo/Discord';
import profile from '@/public/profil.avif';
import Link from 'next/link';
import { AnimatedHeader } from '@/components/text/TextWrite';
import { motion } from 'framer-motion';

const fadeInUp = {
  hidden: { opacity: 0, y: 50 },
  visible: { opacity: 1, y: 0 },
};

const MotionH1 = motion.create('h1');

export default function AnimatedContent() {
  const judul = [
    { text: 'Asep', className: 'text-blue-500 dark:text-blue-500' },
    { text: 'Haryana', className: 'text-blue-500 dark:text-blue-500' },
    { text: 'Saputra', className: 'text-blue-500 dark:text-blue-500' },
  ];
  return (
    <>
      <motion.section
        id='hero'
        initial='hidden'
        animate='visible'
        transition={{ duration: 1 }}
        variants={fadeInUp}
        className='flex items-center justify-center pt-10 bg-lighta dark:bg-darkb min-h-screen'
      >
        <div className='container mx-auto px-6'>
          <div className='flex flex-col lg:flex-row items-center'>
            <motion.div
              className='w-full lg:w-1/2 px-2'
              initial={{ opacity: 0, x: -50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 0.5, duration: 1 }}
            >
              <MotionH1
                className='text-4xl sm:text-5xl md:text-6xl font-semibold text-dark dark:text-lighta'
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.7, duration: 1 }}
              >
                Halo semua 👋, saya <AnimatedHeader words={judul} />
              </MotionH1>
              <motion.p
                className='mt-4 text-lg md:text-xl font-medium text-dark dark:text-lighta'
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.9, duration: 1 }}
              >
                Okelah
              </motion.p>
            </motion.div>
            <motion.div
              className='w-full lg:w-1/2 px-4 mt-10 lg:mt-0 flex justify-center'
              initial={{ opacity: 0, x: 50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 0.5, duration: 1 }}
            >
              <div className='relative w-64 h-64 lg:w-96 lg:h-96'>
                <Image
                  src={profile}
                  alt='Profil'
                  fill
                  sizes='(max-width: 640px) 50vw, (max-width: 1024px) 25vw, 20vw'
                  className='rounded-full object-cover'
                  priority
                  placeholder='blur'
                />
              </div>
            </motion.div>
          </div>
        </div>
      </motion.section>
      <motion.section
        id='about'
        initial='hidden'
        animate='visible'
        transition={{ duration: 1, delay: 1 }}
        variants={fadeInUp}
        className='py-36 bg-lighta dark:bg-darkb'
      >
        <div className='container mx-auto px-6'>
          <div className='flex flex-col lg:flex-row items-center'>
            <motion.div
              className='w-full lg:w-1/2 px-4 mb-10 lg:mb-0'
              initial={{ opacity: 0, x: -50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 1.2, duration: 1 }}
            >
              <h4 className='mb-3 text-lg font-bold uppercase text-dark dark:text-lighta'>
                Tentang Saya
              </h4>
              <h2 className='mb-5 text-2xl sm:text-3xl md:text-4xl font-bold text-dark dark:text-lighta'>
                Saya adalah seorang programmer
              </h2>
              <p className='text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta'>
                Saya adalah programer yang suka belajar hal baru, saya juga suka
                bermain game dan menonton Anime.
              </p>
            </motion.div>
            <motion.div
              className='w-full lg:w-1/2 px-4'
              initial={{ opacity: 0, x: 50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 1.2, duration: 1 }}
            >
              <h3 className='mb-4 text-2xl font-semibold text-dark dark:text-lighta'>
                Mari berteman
              </h3>
              <p className='mb-6 text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta'>
                Berikut adalah beberapa sosial media yang saya punya
              </p>
              <motion.div
                className='flex space-x-4'
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 1.4, duration: 1 }}
              >
                <Link
                  href='https://github.com/MythEclipse'
                  className='flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2'
                >
                  <Discord />
                </Link>
                <Link
                  href='https://www.instagram.com/asepharyana18/'
                  className='flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2'
                >
                  <Instagram />
                </Link>
                <Link
                  href='https://www.linkedin.com/in/asep-haryana-2014a5294/'
                  className='flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2'
                >
                  <Linkedln />
                </Link>
                <Link
                  href='https://www.facebook.com/asep.haryana.900/'
                  className='flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2'
                >
                  <Facebook />
                </Link>
              </motion.div>
            </motion.div>
          </div>
        </div>
      </motion.section>
    </>
  );
}
