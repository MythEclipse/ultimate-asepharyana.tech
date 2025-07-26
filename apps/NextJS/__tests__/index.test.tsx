import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Home from '../app/page';


jest.mock('../components/background/Bg', () => {
  return function MockBg({ children }: { children: React.ReactNode }) {
    return <div data-testid="mock-bg">{children}</div>;
  };
});

jest.mock('../components/landing/AnimatedContent', () => {
  return function MockAnimatedContent() {
    return <div data-testid="mock-animated-content" />;
  };
});

describe('Home', () => {
  it('renders a heading', () => {
    render(<Home />);
    const heading = screen.getByRole('heading', { level: 1 });
    expect(heading).toBeInTheDocument();
  });
});