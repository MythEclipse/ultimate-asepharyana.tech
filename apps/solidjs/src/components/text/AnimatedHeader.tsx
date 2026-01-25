import { cn } from '~/lib/utils';
import { createSignal, createEffect, onCleanup, For } from 'solid-js';

type WordItem = {
  text: string;
  class?: string;
};

export function AnimatedHeader(props: {
  words: WordItem[];
  class?: string;
  cursorClass?: string;
}) {
  const wordsArray = () =>
    props.words.map((word) => ({
      ...word,
      chars: word.text.split(''),
    }));

  const [isInView, setIsInView] = createSignal(false);
  let headerRef: HTMLDivElement | undefined;

  createEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => setIsInView(entry.isIntersecting),
      { threshold: 0.1 },
    );

    const currentHeaderRef = headerRef;
    if (currentHeaderRef) observer.observe(currentHeaderRef);

    onCleanup(() => {
      if (currentHeaderRef) observer.unobserve(currentHeaderRef);
    });
  });

  const fullText = () => props.words.map((w) => w.text).join(' ');

  let globalIndex = 0;

  return (
    <span class={cn('inline-block', props.class)}>
      <span class="sr-only">{fullText()}</span>
      <div ref={headerRef} class="inline" aria-hidden="true">
        <For each={wordsArray()}>
          {(word) => (
            <div class="inline-block">
              <For each={word.chars}>
                {(char) => {
                  const charDelay = globalIndex * 100;
                  globalIndex++;
                  return (
                    <span
                      class={cn(
                        'dark:text-white text-foreground transition-opacity duration-300 ease-in-out',
                        isInView()
                          ? 'opacity-100 translate-y-0'
                          : 'opacity-0 translate-y-2',
                        word.class,
                      )}
                      style={{ 'transition-delay': `${charDelay}ms` }}
                      aria-hidden="true"
                    >
                      {char}
                    </span>
                  );
                }}
              </For>
              &nbsp;
            </div>
          )}
        </For>
      </div>
      <span
        class={cn(
          'inline-block rounded-sm w-[3px] h-7 bg-blue-500 animate-blink ml-1',
          props.cursorClass,
        )}
        aria-hidden="true"
      />
    </span>
  );
}

export default AnimatedHeader;
