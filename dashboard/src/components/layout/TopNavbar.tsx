
import { Search, Bell, RefreshCw, Moon, User } from 'lucide-react';

export function TopNavbar() {
  return (
    <div className="h-16 border-b border-border bg-background/80 backdrop-blur-md flex items-center justify-between px-6 sticky top-0 z-10">
      <div className="flex items-center flex-1">
        <div className="relative w-96">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-secondary" size={18} />
          <input 
            type="text" 
            placeholder="Search agents, policies, logs..." 
            className="w-full bg-surface border border-border rounded-lg pl-10 pr-4 py-2 text-sm text-primary placeholder-secondary focus:outline-none focus:border-accent-indigo focus:ring-1 focus:ring-accent-indigo transition-all"
          />
        </div>
      </div>

      <div className="flex items-center space-x-6">
        <div className="flex items-center space-x-2 text-sm border-r border-border pr-6">
          <span className="text-secondary">Cluster Status:</span>
          <span className="text-accent-teal flex items-center">
            <span className="w-2 h-2 rounded-full bg-accent-teal mr-2 shadow-glow-teal" />
            Quorum Reached
          </span>
        </div>

        <div className="flex items-center space-x-4">
          <button className="text-secondary hover:text-accent-teal transition-colors">
            <RefreshCw size={20} />
          </button>
          <button className="text-secondary hover:text-accent-teal transition-colors relative">
            <Bell size={20} />
            <span className="absolute -top-1 -right-1 w-2 h-2 bg-accent-indigo rounded-full shadow-glow-indigo"></span>
          </button>
          <button className="text-secondary hover:text-accent-teal transition-colors">
            <Moon size={20} />
          </button>
          <div className="w-8 h-8 rounded-full bg-surface border border-border flex items-center justify-center cursor-pointer hover:border-accent-teal transition-colors">
            <User size={16} className="text-primary" />
          </div>
        </div>
      </div>
    </div>
  );
}
