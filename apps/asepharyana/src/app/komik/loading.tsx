import Loading from '@/components/misc/loading';
import React, { memo } from 'react';

const loading = memo(() => {
  return <Loading></Loading>;
});
loading.displayName = 'LoadingPage';

export default loading;
