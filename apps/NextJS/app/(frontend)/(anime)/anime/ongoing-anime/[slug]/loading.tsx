import AnimeGrid from "@/components/card/AnimeGrid";
import { Clapperboard, ArrowRight } from "lucide-react";

const Loading = () => {
    return (
        <main className='p-4 md:p-8 bg-background dark:bg-dark min-h-screen'>
            <div className='max-w-7xl mx-auto'>
                {/* <h1 className='text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent dark:from-blue-400 dark:to-purple-400'>
                    Anime
                </h1> */}

                {/* Ongoing Anime Section */}
                <section className='mb-12 space-y-6'>
                    <div className='flex items-center justify-between mb-6'>
                        <div className='flex items-center gap-4'>
                            <div className='p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl'>
                                <Clapperboard className='w-8 h-8 text-blue-600 dark:text-blue-400' />
                            </div>
                            <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
                                Currently Airing Anime
                            </h1>
                        </div>
                        <div className='flex items-center gap-2 text-blue-600 dark:text-blue-400'>
                            <span className='skeleton w-16 h-4 rounded'></span>
                            <ArrowRight className='w-4 h-4' />
                        </div>
                    </div>

                    <AnimeGrid
                        animes={[]}
                        loading={true}
                    />
                </section>

                {/* Complete Anime Section */}
                {/* <section className='space-y-6'>
                    <div className='flex items-center justify-between mb-6'>
                        <div className='flex items-center gap-3'>
                            <div className='p-3 bg-green-100 dark:bg-green-900/50 rounded-xl'>
                                <CheckCircle className='w-6 h-6 text-green-600 dark:text-green-400' />
                            </div>
                            <h2 className='text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent'>
                                Complete Anime
                            </h2>
                        </div>
                        <div className='flex items-center gap-2 text-green-600 dark:text-green-400'>
                            <span className='skeleton w-16 h-4 rounded'></span>
                            <ArrowRight className='w-4 h-4' />
                        </div>
                    </div>

                    <AnimeGrid
                        animes={[]}
                        loading={true}
                    />
                </section> */}
            </div>
        </main>
    );
};

export default Loading;
