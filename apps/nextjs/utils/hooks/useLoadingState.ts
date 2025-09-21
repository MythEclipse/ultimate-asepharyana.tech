import { useState } from 'react';

export function useLoadingState() {
  const [loading, setLoading] = useState<Record<string, boolean>>({});

  const setLoadingFor = (key: string, value: boolean) =>
    setLoading((prev) => ({ ...prev, [key]: value }));

  return { loading, setLoadingFor };
}
