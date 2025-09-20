import { useState, useCallback } from 'react';

const ITEMS_PER_PAGE = 10;

export const usePagination = (initialPage = 1) => {
  const [pagination, setPagination] = useState({
    currentPage: initialPage,
    itemsPerPage: ITEMS_PER_PAGE,
  });

  const handlePageChange = useCallback((page: number) => {
    setPagination((prev) => ({ ...prev, currentPage: page }));
  }, []);

  return {
    pagination,
    handlePageChange,
    ITEMS_PER_PAGE,
  };
};
