/* eslint-disable @typescript-eslint/no-explicit-any */
// app/api-docs/SwaggerUIWrapper.tsx
'use client';

import SwaggerUI from 'swagger-ui-react';
import 'swagger-ui-react/swagger-ui.css';
import type { OpenAPIV3 } from 'openapi-types';
import { useEffect } from 'react';

interface SwaggerUIWrapperProps {
  spec: OpenAPIV3.Document;
}

export default function SwaggerUIWrapper({ spec }: SwaggerUIWrapperProps) {
  useEffect(() => {
    const originalWarn = console.warn;
    const originalError = console.error;

    console.warn = (...args: any[]) => {
      if (/UNSAFE_componentWillReceiveProps|deprecated|legacy/.test(args[0]))
        return;
      originalWarn(...args);
    };

    console.error = (...args: any[]) => {
      if (/useLayoutEffect|does not support server rendering/.test(args[0]))
        return;
      originalError(...args);
    };

    return () => {
      console.warn = originalWarn;
      console.error = originalError;
    };
  }, []);

  return (
    <SwaggerUI
      spec={spec}
      tryItOutEnabled={true}
      persistAuthorization={true}
      layout='BaseLayout'
      deepLinking={true}
      filter={true}
      requestSnippetsEnabled={true}
    />
  );
}
