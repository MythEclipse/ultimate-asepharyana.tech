'use client';

import { cn } from '../../utils/utils';
import React, { useRef, useState, useLayoutEffect, useEffect } from 'react';

type BeamOptions = {
  initialX: number;
  duration: number;
  delay: number;
  height: number;
};

export const BackgroundBeamsWithCollision = ({
  children,
  className,
  beamCount = 20,
}: {
  children: React.ReactNode;
  className?: string;
  beamCount?: number;
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const parentRef = useRef<HTMLDivElement>(null);
  const [beams, setBeams] = useState<BeamOptions[]>([]);
  const [containerHeight, setContainerHeight] = useState(0);

  // This effect sets up the ResizeObserver to keep track of the container's height.
  useLayoutEffect(() => {
    const parentElement = parentRef.current;
    if (!parentElement) return;

    const updateContainerHeight = () => {
      setContainerHeight(parentElement.scrollHeight);
    };

    updateContainerHeight(); // Initial height update

    const resizeObserver = new ResizeObserver(updateContainerHeight);
    resizeObserver.observe(parentElement);

    return () => {
      resizeObserver.disconnect();
    };
  }, []);

  // This effect regenerates beams when the container height or beam count changes.
  useEffect(() => {
    const parentElement = parentRef.current;
    if (!parentElement || containerHeight === 0) return;

    const parentRect = parentElement.getBoundingClientRect();
    const newBeams = Array.from({ length: beamCount }, () => ({
      initialX: Math.random() * parentRect.width,
      // ADJUSTED: Increased speed factor for an even faster fall.
      duration: containerHeight / 180 + Math.random() * 2,
      delay: Math.random() * 8,
      height: Math.floor(Math.random() * 40 + 60),
    }));
    setBeams(newBeams);
  }, [beamCount, containerHeight]);

  return (
    <div ref={parentRef} className={cn('relative overflow-hidden', className)}>
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
      <div className="absolute inset-0 pointer-events-none z-20">
        {beams.map((beam, index) => (
          <CollisionMechanism
            key={index}
            beamOptions={beam}
            containerRef={containerRef}
            parentRef={parentRef}
            parentHeight={containerHeight}
          />
        ))}
      </div>
      <div ref={containerRef} style={{ position: 'relative', zIndex: 10 }}>
        {children}
      </div>
    </div>
  );
};

const CollisionMechanism = ({
  containerRef,
  parentRef,
  beamOptions = {},
  parentHeight,
}: {
  containerRef: React.RefObject<HTMLDivElement | null>;
  parentRef: React.RefObject<HTMLDivElement | null>;
  beamOptions?: Partial<BeamOptions>;
  parentHeight: number;
}) => {
  const beamRef = useRef<HTMLDivElement>(null);
  const [collision, setCollision] = useState<{
    detected: boolean;
    coordinates: { x: number; y: number } | null;
  }>({ detected: false, coordinates: null });

  const handleAnimation = () => {
    if (Math.random() < 0.5) {
      return;
    }

    const parent = parentRef.current;
    const container = containerRef.current;
    const beam = beamRef.current;
    if (!parent || !container || !beam) return;

    const parentRect = parent.getBoundingClientRect();
    const containerRect = container.getBoundingClientRect();

    const explosionY =
      containerRect.top - parentRect.top + containerRect.height;
    const explosionX = (beamOptions.initialX ?? 0) + beam.offsetWidth / 2;

    setCollision({
      detected: true,
      coordinates: { x: explosionX, y: explosionY },
    });

    // ADJUSTED: Increased timeout for explosion to linger longer.
    setTimeout(
      () => setCollision({ detected: false, coordinates: null }),
      2000,
    );
  };

  return (
    <>
      <div
        ref={beamRef}
        className="absolute w-px bg-gradient-to-t from-indigo-500/0 via-purple-500 to-indigo-500/0 animation-beam"
        style={
          {
            left: `${beamOptions.initialX ?? 0}px`,
            height: `${beamOptions.height ?? 80}px`,
            boxShadow: '0px 0px 10px 1px rgba(139, 92, 246, 0.6)',
            animationName: 'beam',
            animationDuration: `${beamOptions.duration ?? 10}s`, // Default duration if height is 0
            animationDelay: `${beamOptions.delay ?? 0}s`,
            animationFillMode: 'backwards',
            animationTimingFunction: 'linear',
            animationIterationCount: 'infinite',
            '--beam-end-y': `${parentHeight}px`,
          } as React.CSSProperties
        }
        onAnimationIteration={handleAnimation}
      />
      {collision.detected && collision.coordinates && (
        <Explosion
          style={{
            left: `${collision.coordinates.x}px`,
            top: `${collision.coordinates.y}px`,
            transform: 'translate(-50%, -50%)',
          }}
        />
      )}
    </>
  );
};

const Explosion = ({ ...props }: React.HTMLProps<HTMLDivElement>) => {
  const particles = Array.from({ length: 20 }, (_, index) => ({
    id: index,
    directionX: Math.random() * 2 - 1,
    directionY: Math.random() * 2 - 1,
    distance: Math.random() * 80 + 40,
  }));

  return (
    <div
      {...props}
      className={cn('absolute z-50 pointer-events-none', props.className)}
    >
      {particles.map((particle) => (
        <span
          key={particle.id}
          className="absolute animation-explosion-particle"
          style={
            {
              backgroundImage:
                'radial-gradient(circle, rgba(167, 139, 250, 0.8) 0%, rgba(167, 139, 250, 0) 70%)',
              width: '10px',
              height: '10px',
              borderRadius: '50%',
              '--distance-x': `${particle.directionX * particle.distance}px`,
              '--distance-y': `${particle.directionY * particle.distance}px`,
              animationName: 'explosion-particle',
              // ADJUSTED: Increased particle animation duration.
              animationDuration: '1.5s',
              animationTimingFunction: 'ease-out',
              animationFillMode: 'forwards',
              animationDelay: `${Math.random() * 0.2}s`,
            } as React.CSSProperties
          }
        />
      ))}
    </div>
  );
};

export default BackgroundBeamsWithCollision;
