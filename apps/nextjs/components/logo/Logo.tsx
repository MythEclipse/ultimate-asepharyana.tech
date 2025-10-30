'use client';

import React, { memo } from 'react';
import Link from 'next/link';
import Image from 'next/image';

/**
 * Logo component - Shared logo for navbar
 * Displays site logo with text on larger screens
 */
const Logo = memo(() => (
  <Link href="/" className="flex items-center gap-2">
    <Image src="/Logo.svg" alt="Logo" width={28} height={28} />
    <span className="hidden text-base font-semibold sm:inline-block md:text-lg">
      Asep Haryana
    </span>
  </Link>
));

Logo.displayName = 'Logo';

export default Logo;
