'use client';
import { cn } from '@/lib/utils';
import React, { useRef, useState, useEffect } from 'react';

export const BackgroundBeamsWithCollision = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const parentRef = useRef<HTMLDivElement>(null);
  const [beams, setBeams] = useState<
    { initialX: number; duration: number; delay: number; height: number }[]
  >([]);

  useEffect(() => {
    if (!parentRef.current) return;
    
    const parentRect = parentRef.current.getBoundingClientRect();
    const newBeams = Array.from({ length: 10 }, () => ({
      initialX: Math.random() * parentRect.width,
      duration: Math.random() * 8 + 4,
      delay: Math.random() * 4,
      height: Math.floor(Math.random() * 40 + 20), // Height between 20-60px
    }));
    setBeams(newBeams);
  }, []);

  return (
    <div ref={parentRef} className={cn('relative overflow-hidden', className)}>
      <style>{`
        @keyframes animate-beam {
          0% { transform: translateY(-200px); }
          100% { transform: translateY(calc(100vh + 200px)); }
        }
        @keyframes particle-explosion {
          to {
            transform: translate(var(--distance-x), var(--distance-y));
            opacity: 0;
          }
        }
      `}</style>
      <div className='absolute inset-0 pointer-events-none z-20'>
        {beams.map((beam, index) => (
          <CollisionMechanism
            key={index}
            beamOptions={beam}
            containerRef={containerRef}
            parentRef={parentRef}
          />
        ))}
      </div>
      <div ref={containerRef} style={{ position: 'relative', zIndex: 10 }}>
        {children}
      </div>
    </div>
  );
};

const CollisionMechanism = React.forwardRef<
  HTMLDivElement,
  {
    containerRef: React.RefObject<HTMLDivElement | null>;
    parentRef: React.RefObject<HTMLDivElement | null>;
    beamOptions?: {
      initialX?: number;
      duration?: number;
      delay?: number;
      height?: number;
    };
  }
>(({ beamOptions = {}, containerRef, parentRef }, ref) => {
  const beamRef = useRef<HTMLDivElement>(null);
  const [collision, setCollision] = useState<{
    detected: boolean;
    coordinates: { x: number; y: number } | null;
  }>({ detected: false, coordinates: null });

  const handleAnimationEnd = () => {
    const beam = beamRef.current;
    const parent = parentRef.current;
    if (!beam || !parent) return;

    const beamRect = beam.getBoundingClientRect();
    const parentRect = parent.getBoundingClientRect();
    
    setCollision({
      detected: true,
      coordinates: {
        x: beamRect.left - parentRect.left + beamRect.width/2,
        y: beamRect.top - parentRect.top + beamRect.height
      }
    });
    
    setTimeout(() => setCollision({ detected: false, coordinates: null }), 1500);
  };

  return (
    <>
      <div
        ref={beamRef}
        className="absolute w-px rounded-full bg-gradient-to-t from-indigo-500 via-purple-500 to-transparent animate-beam"
        style={{
          left: `${beamOptions.initialX || 0}px`,
          height: `${beamOptions.height || 56}px`,
          animationDuration: `${beamOptions.duration || 8}s`,
          animationDelay: `${beamOptions.delay || 0}s`,
        }}
        onAnimationEnd={handleAnimationEnd}
      />
      {collision.detected && collision.coordinates && (
        <Explosion
          className="absolute"
          style={{
            left: `${collision.coordinates.x}px`,
            top: `${collision.coordinates.y}px`,
            transform: 'translate(-50%, -50%)',
          }}
        />
      )}
    </>
  );
});

CollisionMechanism.displayName = 'CollisionMechanism';

const Explosion = ({ ...props }: React.HTMLProps<HTMLDivElement>) => {
  const spans = Array.from({ length: 20 }, (_, index) => ({
    id: index,
    directionX: Math.random() * 2 - 1,
    directionY: Math.random() * 2 - 1,
    distance: Math.random() * 80 + 40,
  }));

  return (
    <div {...props} className={cn('absolute z-50', props.className)}>
      {spans.map((span) => (
        <span
          key={span.id}
          className="absolute h-2 w-2 rounded-full bg-gradient-to-b from-indigo-500 to-purple-500"
          style={{
            animation: 'particle-explosion 1s ease-out forwards',
            animationDelay: `${Math.random() * 0.2}s`,
            transform: 'translate(0, 0)',
            opacity: 1,
            willChange: 'transform, opacity',
            '--distance-x': `${span.directionX * span.distance}px`,
            '--distance-y': `${span.directionY * span.distance}px`,
          } as React.CSSProperties}
        />
      ))}
    </div>
  );
};