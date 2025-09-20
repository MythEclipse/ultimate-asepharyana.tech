import React from 'react';
import SearchForm from './SearchForm';

import type { SearchFormProps } from './SearchForm';

// Assign to variable before export default to fix lint warning
const meta = {
  title: 'Components/SearchForm',
  component: SearchForm,
  argTypes: {
    initialQuery: { control: 'text' },
    baseUrl: { control: 'text' },
    classname: { control: 'text' },
    page: { control: 'text' },
  },
};
export default meta;

export const Default = {
  args: {
    initialQuery: 'Naruto',
    baseUrl: '/anime',
    classname: 'storybook-search-form',
    page: '1',
  } as SearchFormProps,
  render: (args: SearchFormProps) => <SearchForm {...args} />,
};
