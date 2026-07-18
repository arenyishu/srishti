import React from 'react';
import { Sidebar } from './Sidebar';
import { TopNavbar } from './TopNavbar';

export function DashboardLayout({ children, activeView, onNavigate }: { children: React.ReactNode, activeView?: string, onNavigate?: (view: string) => void }) {
  return (
    <div className="flex h-screen bg-background text-primary overflow-hidden">
      <Sidebar activeView={activeView} onNavigate={onNavigate} />
      <div className="flex-1 flex flex-col min-w-0">
        <TopNavbar />
        <main className="flex-1 overflow-y-auto p-6 scrollbar-hide">
          {children}
        </main>
      </div>
    </div>
  );
}
