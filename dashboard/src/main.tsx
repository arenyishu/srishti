import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import App from './App.tsx';
import { SrishtiProvider } from './contexts/SrishtiContext.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <SrishtiProvider>
      <App />
    </SrishtiProvider>
  </StrictMode>,
);
