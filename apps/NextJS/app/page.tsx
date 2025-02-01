import React from 'react'
import Image from 'next/image'
import { Metadata } from 'next'
import Instagram from '@/components/logo/Instagram'
import Facebook from '@/components/logo/Facebook'
import Linkedln from '@/components/logo/LinkedIn'
import Discord from '@/components/logo/Discord'
import Link from 'next/link'
import Bg from '@/components/background/Bg'
import { AnimatedHeader } from '@/components/text/TextWrite'

export const metadata: Metadata = {
  title: 'Home'
}

export default function Home() {
  const judul = [
    { text: 'Asep', className: 'text-blue-500 dark:text-blue-500' },
    { text: 'Haryana', className: 'text-blue-500 dark:text-blue-500' },
    { text: 'Saputra', className: 'text-blue-500 dark:text-blue-500' }
  ]
  return (
    <main>
      <Bg>
        <section id="hero" className="flex items-center justify-center pt-10 bg-lighta dark:bg-darkb min-h-screen">
          <div className="container mx-auto px-6">
            <div className="flex flex-col lg:flex-row items-center">
              <div className="w-full lg:w-1/2 px-2">
                <h1 className="text-4xl sm:text-5xl md:text-6xl font-semibold text-dark dark:text-lighta">
                  Halo semua 👋, saya <AnimatedHeader words={judul} />
                </h1>
                <p className="mt-4 text-lg md:text-xl font-medium text-dark dark:text-lighta">
                  Okelah
                </p>
              </div>
              <div className="w-full lg:w-1/2 px-4 mt-10 lg:mt-0 flex justify-center">
                <div className="relative w-64 h-64 lg:w-96 lg:h-96">
                  <Image
                    src="/profil.jpg"
                    alt="Profil"
                    fill
                    sizes="100vw"
                    className="rounded-full object-cover"
                    priority
                  />
                </div>
              </div>
            </div>
          </div>
        </section>
        <section id="about" className="py-36 bg-lighta dark:bg-darkb">
          <div className="container mx-auto px-6">
            <div className="flex flex-col lg:flex-row items-center">
              <div className="w-full lg:w-1/2 px-4 mb-10 lg:mb-0">
                <h4 className="mb-3 text-lg font-bold uppercase text-dark dark:text-lighta">
                  Tentang Saya
                </h4>
                <h2 className="mb-5 text-2xl sm:text-3xl md:text-4xl font-bold text-dark dark:text-lighta">
                  Saya adalah seorang programmer
                </h2>
                <p className="text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta">
                  Saya adalah programer yang suka belajar hal baru, saya juga suka bermain game dan
                  menonton Anime.
                </p>
              </div>
              <div className="w-full lg:w-1/2 px-4">
                <h3 className="mb-4 text-2xl font-semibold text-dark dark:text-lighta">
                  Mari berteman
                </h3>
                <p className="mb-6 text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta">
                  Berikut adalah beberapa sosial media yang saya punya
                </p>
                <div className="flex space-x-4">
                  <Link
                    href="https://github.com/MythEclipse"
                    className="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                  >
                    <Discord />
                  </Link>
                  <Link
                    href="https://www.instagram.com/asepharyana18/"
                    className="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                  >
                    <Instagram />
                  </Link>
                  <Link
                    href="https://www.linkedin.com/in/asepharyana/"
                    className="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                  >
                    <Linkedln />
                  </Link>
                  <Link
                    href="https://www.facebook.com/asep.haryana.900/"
                    className="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                  >
                    <Facebook />
                  </Link>
                </div>
              </div>
            </div>
          </div>
        </section>
      </Bg>
    </main>
  )
}
