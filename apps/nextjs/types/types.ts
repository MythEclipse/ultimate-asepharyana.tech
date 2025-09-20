export interface Pagination {
  current_page?: number;
  last_visible_page?: number;
  has_next_page?: boolean;
  next_page?: number | null;
  has_previous_page?: boolean;
  previous_page?: number | null;
}

export interface NavLink {
  href: string;
  label: string;
}
