import { render } from '@testing-library/react';

import StorybookUi from './storybook-ui';

describe('StorybookUi', () => {
  it('should render successfully', () => {
    const { baseElement } = render(<StorybookUi />);
    expect(baseElement).toBeTruthy();
  });
});
