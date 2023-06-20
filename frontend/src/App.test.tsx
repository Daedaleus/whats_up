import React from 'react'
import { render, screen } from '@testing-library/react'
import App from './App'

test('render the app', (): void => {
  render(<App/>)
  const linkElement = screen.getByText(/Whats up?/i)
  expect(linkElement).toEqual('Whats up?')
})
