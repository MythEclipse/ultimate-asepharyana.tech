/**
 * Input sanitization utilities
 */

/**
 * Sanitize string input to prevent XSS
 */
export const sanitizeString = (input: string): string => {
  return input
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;')
    .replace(/\//g, '&#x2F;');
};

/**
 * Validate and sanitize email
 */
export const sanitizeEmail = (email: string): string | null => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  const trimmed = email.trim().toLowerCase();

  if (!emailRegex.test(trimmed)) {
    return null;
  }

  return trimmed;
};

/**
 * Sanitize object recursively
 */
export const sanitizeObject = <T extends Record<string, unknown>>(
  obj: T,
): T => {
  const sanitized = {} as T;

  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === 'string') {
      sanitized[key as keyof T] = sanitizeString(value) as T[keyof T];
    } else if (
      typeof value === 'object' &&
      value !== null &&
      !Array.isArray(value)
    ) {
      sanitized[key as keyof T] = sanitizeObject(
        value as Record<string, unknown>,
      ) as T[keyof T];
    } else {
      sanitized[key as keyof T] = value as T[keyof T];
    }
  }

  return sanitized;
};

/**
 * Validate pagination parameters
 */
export const validatePagination = (page?: unknown, limit?: unknown) => {
  const pageNum = typeof page === 'number' ? page : parseInt(String(page || 1));
  const limitNum =
    typeof limit === 'number' ? limit : parseInt(String(limit || 10));

  if (isNaN(pageNum) || pageNum < 1) {
    throw new Error('Invalid page parameter');
  }

  if (isNaN(limitNum) || limitNum < 1 || limitNum > 100) {
    throw new Error('Invalid limit parameter (must be between 1 and 100)');
  }

  return { page: pageNum, limit: limitNum };
};
