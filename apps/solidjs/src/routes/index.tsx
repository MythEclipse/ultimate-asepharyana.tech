import { Title } from "@solidjs/meta";
import { Motion } from "solid-motionone";
import { A } from "@solidjs/router";
import BackgroundBeamsWithCollision from "~/components/background/BackgroundBeamsWithCollision";
import { AnimatedHeader } from "~/components/text/AnimatedHeader";
import { Instagram, Facebook, LinkedIn, GitHub } from "~/components/logo/SocialIcons";

const fadeInUp = {
  hidden: { opacity: 0, y: 50 },
  visible: { opacity: 1, y: 0 },
};

const judul = [
  { text: "Asep", class: "text-blue-500 dark:text-blue-500" },
  { text: "Haryana", class: "text-blue-500 dark:text-blue-500" },
  { text: "Saputra", class: "text-blue-500 dark:text-blue-500" },
];

export default function Home() {
  return (
    <>
      <Title>Home | Asepharyana</Title>
      <main class="min-h-screen bg-background text-foreground">
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-64px)] p-4 md:p-8 lg:p-12">
          <BackgroundBeamsWithCollision>
            {/* Hero Section */}
            <Motion.section
              initial={fadeInUp.hidden}
              animate={fadeInUp.visible}
              transition={{ duration: 1 }}
              class="flex items-center justify-center pt-10 bg-lighta dark:bg-darkb min-h-screen"
            >
              <div class="container mx-auto px-6">
                <div class="flex flex-col lg:flex-row items-center">
                  <Motion.div
                    class="w-full lg:w-1/2 px-2"
                    initial={{ opacity: 0, x: -50 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: 0.5, duration: 1 }}
                  >
                    <Motion.h1
                      class="text-4xl sm:text-5xl md:text-6xl font-semibold text-dark dark:text-lighta"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ delay: 0.7, duration: 1 }}
                    >
                      Halo semua 👋, saya <AnimatedHeader words={judul} />
                    </Motion.h1>
                    <Motion.p
                      class="mt-4 text-lg md:text-xl font-medium text-dark dark:text-lighta"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ delay: 0.9, duration: 1 }}
                    >
                      Okelah
                    </Motion.p>
                  </Motion.div>
                  <Motion.div
                    class="w-full lg:w-1/2 px-4 mt-10 lg:mt-0 flex justify-center"
                    initial={{ opacity: 0, x: 50 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: 0.5, duration: 1 }}
                  >
                    <div class="relative w-64 h-64 lg:w-96 lg:h-96">
                      <img
                        src="/profil.avif"
                        alt="Profil"
                        class="rounded-full object-cover w-full h-full"
                      />
                    </div>
                  </Motion.div>
                </div>
              </div>
            </Motion.section>

            {/* About Section */}
            <Motion.section
              initial={fadeInUp.hidden}
              animate={fadeInUp.visible}
              transition={{ duration: 1, delay: 1 }}
              class="py-36 bg-lighta dark:bg-darkb"
            >
              <div class="container mx-auto px-6">
                <div class="flex flex-col lg:flex-row items-center">
                  <Motion.div
                    class="w-full lg:w-1/2 px-4 mb-10 lg:mb-0"
                    initial={{ opacity: 0, x: -50 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: 1.2, duration: 1 }}
                  >
                    <h4 class="mb-3 text-lg font-bold uppercase text-dark dark:text-lighta">
                      Tentang Saya
                    </h4>
                    <h2 class="mb-5 text-2xl sm:text-3xl md:text-4xl font-bold text-dark dark:text-lighta">
                      Saya adalah seorang programmer
                    </h2>
                    <p class="text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta">
                      Saya adalah programer yang suka belajar hal baru, saya juga suka
                      bermain game dan menonton Anime.
                    </p>
                  </Motion.div>
                  <Motion.div
                    class="w-full lg:w-1/2 px-4"
                    initial={{ opacity: 0, x: 50 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: 1.2, duration: 1 }}
                  >
                    <h3 class="mb-4 text-2xl font-semibold text-dark dark:text-lighta">
                      Mari berteman
                    </h3>
                    <p class="mb-6 text-base sm:text-lg md:text-xl font-medium text-dark dark:text-lighta">
                      Berikut adalah beberapa sosial media yang saya punya
                    </p>
                    <Motion.div
                      class="flex space-x-4"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ delay: 1.4, duration: 1 }}
                    >
                      <A
                        href="https://github.com/MythEclipse"
                        class="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <GitHub />
                      </A>
                      <A
                        href="https://www.instagram.com/asepharyana18/"
                        class="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <Instagram />
                      </A>
                      <A
                        href="https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/"
                        class="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <LinkedIn />
                      </A>
                      <A
                        href="https://www.facebook.com/asep.haryana.900/"
                        class="flex items-center justify-center rounded-full border border-slate-300 text-dark hover:border-primary-600 hover:bg-primary-600 hover:text-lighta dark:text-lighta p-2"
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <Facebook />
                      </A>
                    </Motion.div>
                  </Motion.div>
                </div>
              </div>
            </Motion.section>
          </BackgroundBeamsWithCollision>
        </div>
      </main>
    </>
  );
}
