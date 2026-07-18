import React from 'react';
import { 
  LayoutDashboard, 
  Cpu, 
  Network, 
  Database, 
  ShieldCheck, 
  CheckSquare, 
  ScrollText, 
  Activity, 
  Package, 
  Rocket, 
  Store, 
  Settings,
  GitMerge
} from 'lucide-react';
import { cn } from '../../lib/utils';

interface NavItemProps {
  icon: React.ElementType;
  label: string;
  active?: boolean;
  onClick?: () => void;
}

const NavItem = ({ icon: Icon, label, active, onClick }: NavItemProps) => (
  <button 
    onClick={onClick}
    className={cn(
      "w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 group",
      active 
        ? "bg-accent-indigo/20 text-accent-teal border border-accent-indigo/30 shadow-glow-indigo" 
        : "text-secondary hover:bg-surface hover:text-primary hover:border hover:border-border"
    )}
  >
    <Icon size={20} className={cn("transition-colors", active ? "text-accent-teal" : "group-hover:text-accent-teal")} />
    <span className="font-medium tracking-wide">{label}</span>
  </button>
);

export function Sidebar({ activeView, onNavigate }: { activeView?: string, onNavigate?: (view: string) => void }) {
  const nav = (label: string) => () => {
    if (onNavigate) onNavigate(label);
  };

  return (
    <div className="h-screen w-64 bg-background border-r border-border flex flex-col pt-6 flex-shrink-0">
      <div className="px-6 mb-8">
        <div className="flex items-center space-x-3 mb-2">
          <div className="w-8 h-8 rounded-full bg-accent-indigo flex items-center justify-center shadow-glow-indigo">
            <div className="w-4 h-4 bg-accent-teal rounded-full" />
          </div>
          <h1 className="text-2xl font-bold tracking-wider text-primary neon-text">Srishti OS</h1>
        </div>
        <p className="text-xs text-secondary tracking-widest uppercase">The OS for AI Agents</p>
      </div>

      <div className="flex-1 overflow-y-auto px-3 space-y-1 scrollbar-hide">
        <NavItem icon={LayoutDashboard} label="Overview" active={activeView === 'Overview'} onClick={nav('Overview')} />
        <NavItem icon={Cpu} label="Agents" active={activeView === 'Agents'} onClick={nav('Agents')} />
        <NavItem icon={GitMerge} label="Workflows" active={activeView === 'Workflows'} onClick={nav('Workflows')} />
        <NavItem icon={Network} label="Cluster" active={activeView === 'Cluster'} onClick={nav('Cluster')} />
        <NavItem icon={Database} label="Memory" active={activeView === 'Memory'} onClick={nav('Memory')} />
        <NavItem icon={ShieldCheck} label="Policies" active={activeView === 'Policies'} onClick={nav('Policies')} />
        <NavItem icon={CheckSquare} label="Approvals" active={activeView === 'Approvals'} onClick={nav('Approvals')} />
        <NavItem icon={ScrollText} label="Audit Logs" active={activeView === 'Audit Logs'} onClick={nav('Audit Logs')} />
        <NavItem icon={Activity} label="Debug" active={activeView === 'Debug'} onClick={nav('Debug')} />
        <NavItem icon={Activity} label="Metrics" active={activeView === 'Metrics'} onClick={nav('Metrics')} />
        <NavItem icon={Package} label="Packages" active={activeView === 'Packages'} onClick={nav('Packages')} />
        <NavItem icon={Rocket} label="Deployments" active={activeView === 'Deployments'} onClick={nav('Deployments')} />
        <NavItem icon={Store} label="Marketplace" active={activeView === 'Marketplace'} onClick={nav('Marketplace')} />
        <NavItem icon={Settings} label="Settings" active={activeView === 'Settings'} onClick={nav('Settings')} />
      </div>

      <div className="p-4 mt-auto border-t border-border">
        <div className="glass-panel p-4 flex flex-col space-y-3 text-sm">
          <div className="flex justify-between items-center">
            <span className="text-secondary">Status</span>
            <span className="flex items-center text-accent-teal">
              <span className="w-2 h-2 rounded-full bg-accent-teal mr-2 animate-pulse" />
              Operational
            </span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-secondary">Node ID</span>
            <span className="text-primary font-mono text-xs">nd-8f92a</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-secondary">Version</span>
            <span className="text-primary font-mono text-xs">v1.0 Moksha</span>
          </div>
          <button className="w-full mt-2 py-2 rounded-md bg-surface border border-border text-primary hover:border-accent-teal hover:text-accent-teal transition-colors">
            Documentation
          </button>
        </div>
      </div>
    </div>
  );
}
