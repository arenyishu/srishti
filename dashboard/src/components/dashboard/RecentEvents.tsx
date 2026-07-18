import { Terminal, Shield, Zap, AlertTriangle, Cpu } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function RecentEvents() {
  const { events } = useSrishti();

  const getEventIcon = (name: string) => {
    if (name.includes('Suspended') || name.includes('Policy')) return <Shield className="w-4 h-4 text-yellow-400" />;
    if (name.includes('ToolFailed')) return <AlertTriangle className="w-4 h-4 text-rose-400" />;
    if (name.includes('Started')) return <Zap className="w-4 h-4 text-accent-teal" />;
    if (name.includes('Cron')) return <Cpu className="w-4 h-4 text-accent-indigo" />;
    return <Terminal className="w-4 h-4 text-secondary" />;
  };

  return (
    <div className="glass-panel rounded-xl overflow-hidden h-full flex flex-col">
      <div className="p-6 border-b border-border flex justify-between items-center">
        <h3 className="text-xl font-bold text-primary">System Audit Log</h3>
        <span className="flex h-2 w-2 relative">
          <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-accent-teal opacity-75"></span>
          <span className="relative inline-flex rounded-full h-2 w-2 bg-accent-teal"></span>
        </span>
      </div>
      
      <div className="p-4 flex-1 overflow-y-auto font-mono text-sm">
        <div className="space-y-4">
          {events.length === 0 ? (
            <div className="text-secondary text-center p-4">Waiting for events...</div>
          ) : events.map((event, i) => (
            <div key={i} className="flex gap-4 p-3 rounded-lg bg-surface/30 border border-border/50 hover:border-accent-teal/30 transition-colors">
              <div className="mt-1">{getEventIcon(event.name)}</div>
              <div className="flex-1">
                <div className="flex justify-between items-start mb-1">
                  <span className="font-semibold text-primary">{event.name}</span>
                  <span className="text-xs text-secondary opacity-70">
                    {new Date(event.timestamp).toLocaleTimeString()}
                  </span>
                </div>
                <p className="text-secondary mb-2 line-clamp-2">
                  <span className="text-accent-indigo">{event.source_agent}</span>
                  {event.target_agent ? ` -> ${event.target_agent}` : ''}
                </p>
                {Object.keys(event.payload).length > 0 && (
                  <pre className="text-xs text-secondary bg-surface p-2 rounded border border-border overflow-x-auto">
                    {JSON.stringify(event.payload, null, 2)}
                  </pre>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
