import { cn } from "~/lib/utils";
import { createSignal, createEffect, onCleanup, For, Show, JSX, ParentComponent } from "solid-js";

type BeamOptions = {
    initialX: number;
    duration: number;
    delay: number;
    height: number;
};

const CollisionMechanism = (props: {
    containerRef: HTMLDivElement | undefined;
    parentRef: HTMLDivElement | undefined;
    beamOptions: Partial<BeamOptions>;
    parentHeight: number;
}) => {
    let beamRef: HTMLDivElement | undefined;
    const [collision, setCollision] = createSignal<{
        detected: boolean;
        coordinates: { x: number; y: number } | null;
    }>({ detected: false, coordinates: null });

    const handleAnimation = () => {
        if (Math.random() < 0.5) {
            return;
        }

        const parent = props.parentRef;
        const container = props.containerRef;
        const beam = beamRef;
        if (!parent || !container || !beam) return;

        const parentRect = parent.getBoundingClientRect();
        const containerRect = container.getBoundingClientRect();

        const explosionY =
            containerRect.top - parentRect.top + containerRect.height;
        const explosionX = (props.beamOptions.initialX ?? 0) + beam.offsetWidth / 2;

        setCollision({
            detected: true,
            coordinates: { x: explosionX, y: explosionY },
        });

        setTimeout(
            () => setCollision({ detected: false, coordinates: null }),
            2000
        );
    };

    const particles = Array.from({ length: 20 }, (_, index) => ({
        id: index,
        directionX: Math.random() * 2 - 1,
        directionY: Math.random() * 2 - 1,
        distance: Math.random() * 80 + 40,
    }));

    return (
        <>
            <div
                ref={beamRef}
                class="absolute w-px bg-gradient-to-t from-indigo-500/0 via-purple-500 to-indigo-500/0"
                style={{
                    left: `${props.beamOptions.initialX ?? 0}px`,
                    height: `${props.beamOptions.height ?? 80}px`,
                    "box-shadow": "0px 0px 10px 1px rgba(139, 92, 246, 0.6)",
                    "animation-name": "beam",
                    "animation-duration": `${props.beamOptions.duration ?? 10}s`,
                    "animation-delay": `${props.beamOptions.delay ?? 0}s`,
                    "animation-fill-mode": "backwards",
                    "animation-timing-function": "linear",
                    "animation-iteration-count": "infinite",
                    "--beam-end-y": `${props.parentHeight}px`,
                } as JSX.CSSProperties}
                onAnimationIteration={handleAnimation}
            />
            <Show when={collision().detected && collision().coordinates}>
                <div
                    class="absolute z-50 pointer-events-none"
                    style={{
                        left: `${collision().coordinates!.x}px`,
                        top: `${collision().coordinates!.y}px`,
                        transform: "translate(-50%, -50%)",
                    }}
                >
                    <For each={particles}>
                        {(particle) => (
                            <span
                                class="absolute"
                                style={{
                                    "background-image":
                                        "radial-gradient(circle, rgba(167, 139, 250, 0.8) 0%, rgba(167, 139, 250, 0) 70%)",
                                    width: "10px",
                                    height: "10px",
                                    "border-radius": "50%",
                                    "--distance-x": `${particle.directionX * particle.distance}px`,
                                    "--distance-y": `${particle.directionY * particle.distance}px`,
                                    "animation-name": "explosion-particle",
                                    "animation-duration": "1.5s",
                                    "animation-timing-function": "ease-out",
                                    "animation-fill-mode": "forwards",
                                    "animation-delay": `${Math.random() * 0.2}s`,
                                } as JSX.CSSProperties}
                            />
                        )}
                    </For>
                </div>
            </Show>
        </>
    );
};

export const BackgroundBeamsWithCollision: ParentComponent<{
    class?: string;
    beamCount?: number;
}> = (props) => {
    let containerRef: HTMLDivElement | undefined;
    let parentRef: HTMLDivElement | undefined;
    const [beams, setBeams] = createSignal<BeamOptions[]>([]);
    const [containerHeight, setContainerHeight] = createSignal(0);

    createEffect(() => {
        const parentElement = parentRef;
        if (!parentElement) return;

        const updateContainerHeight = () => {
            setContainerHeight(parentElement.scrollHeight);
        };

        updateContainerHeight();

        const resizeObserver = new ResizeObserver(updateContainerHeight);
        resizeObserver.observe(parentElement);

        onCleanup(() => {
            resizeObserver.disconnect();
        });
    });

    createEffect(() => {
        const parentElement = parentRef;
        const height = containerHeight();
        if (!parentElement || height === 0) return;

        const parentRect = parentElement.getBoundingClientRect();
        const beamCount = props.beamCount ?? 20;
        const newBeams = Array.from({ length: beamCount }, () => ({
            initialX: Math.random() * parentRect.width,
            duration: height / 180 + Math.random() * 2,
            delay: Math.random() * 8,
            height: Math.floor(Math.random() * 40 + 60),
        }));
        setBeams(newBeams);
    });

    return (
        <div ref={parentRef} class={cn("relative overflow-hidden", props.class)}>
            <style>{`
        @keyframes beam {
          from {
            transform: translateY(-200px);
            opacity: 1;
          }
          to {
            transform: translateY(var(--beam-end-y, 120vh));
            opacity: 0;
          }
        }
        @keyframes explosion-particle {
          from {
            transform: translate(0, 0);
            opacity: 1;
          }
          to {
            transform: translate(var(--distance-x), var(--distance-y));
            opacity: 0;
          }
        }
      `}</style>
            <div class="absolute inset-0 pointer-events-none z-20">
                <For each={beams()}>
                    {(beam) => (
                        <CollisionMechanism
                            beamOptions={beam}
                            containerRef={containerRef}
                            parentRef={parentRef}
                            parentHeight={containerHeight()}
                        />
                    )}
                </For>
            </div>
            <div ref={containerRef} style={{ position: "relative", "z-index": 10 }}>
                {props.children}
            </div>
        </div>
    );
};

export default BackgroundBeamsWithCollision;
