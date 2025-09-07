'use client';

import { useEffect, useRef, useState } from 'react';
import type { OpenAPIV3 } from 'openapi-types';
import { useTheme } from 'next-themes';
import 'swagger-ui-dist/swagger-ui.css';
import './swagger-dark.css';

import { SwaggerUIBundle } from 'swagger-ui-dist';

interface SwaggerUIComponentProps {
  spec: OpenAPIV3.Document;
}

export default function SwaggerUIComponent({ spec }: SwaggerUIComponentProps) {
  const swaggerUIRef = useRef<HTMLDivElement>(null);
  const { resolvedTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    let cleanupFn: (() => void) | undefined;

    if (swaggerUIRef.current && spec && mounted) {
      const ui = SwaggerUIBundle({
        spec: spec,
        domNode: swaggerUIRef.current,
        deepLinking: true,
        tryItOutEnabled: true,
        persistAuthorization: true,
        requestSnippetsEnabled: true,
        filter: true,
        syntaxHighlight: {
          activate: true,
          theme: resolvedTheme === 'dark' ? 'monokai' : 'agate',
        },
      });

      const currentRef = swaggerUIRef.current;
      cleanupFn = () => {
        if (currentRef) {
          currentRef.innerHTML = '';
        }
        ui.getSystem().specActions.updateSpec('{}');
      };
    }

    return cleanupFn;
  }, [spec, resolvedTheme, mounted]);

  return (
    <div
      className={mounted && resolvedTheme === 'dark' ? 'dark-theme' : ''}
      ref={swaggerUIRef}
    />
  );
}
