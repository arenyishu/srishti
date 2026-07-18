import { ScrollText, ShieldAlert, Zap, Cpu, Terminal } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function AuditCenter() {
  const { events } = useSrishti();

  const getEventIcon = (name: string) => {
    if (name.includes('Suspended') || name.includes('Policy')) return <ShieldAlert className="w-5 h-5 text-yellow-400" />;
    if (name.includes('Failed')) return <Terminal className="w-5 h-5 text-rose-400" />;
    if (name.includes('Started')) return <Zap className="w-5 h-5 text-accent-teal" />;
    if (name.includes('Cron')) return <Cpu className="w-5 h-5 text-accent-indigo" />;
    return <ScrollText className="w-5 h-5 text-secondary" />;
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <ScrollText className="w-8 h-8 text-accent-teal" />
        <h2 className="text-3xl font-bold text-primary">System Audit Log</h2>
      </div>

      <div className="glass-panel rounded-xl overflow-hidden">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-surface/50 text-secondary text-sm">
              <th className="p-4 font-medium border-b border-border">TIMESTAMP</th>
              <th className="p-4 font-medium border-b border-border">EVENT</th>
              <th className="p-4 font-medium border-b border-border">SOURCE AGENT</th>
              <th className="p-4 font-medium border-b border-border">PAYLOAD DETAILS</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border/50">
            {events.length === 0 ? (
              <tr>
                <td colSpan={4} className="p-8 text-center text-secondary">No events recorded.</td>
              </tr>
            ) : events.map((event, i) => (
              <tr key={i} className="hover:bg-surface/30 transition-colors">
                <td className="p-4 text-sm text-secondary font-mono">{new Date(event.timestamp).toLocaleString()}</td>
                <td className="p-4">
                  <div className="flex items-center space-x-2">
                    {getEventIcon(event.name)}
                    <span className="font-semibold text-primary">{event.name}</span>
                  </div>
                </td>
                <td className="p-4 text-sm text-accent-indigo">{event.source_agent}</td>
                <td className="p-4">
                  {Object.keys(event.payload).length > 0 ? (
                    <pre className="text-xs text-secondary bg-background p-2 rounded border border-border overflow-x-auto max-w-lg">
                      {JSON.stringify(event.payload, null, 2)}
                    </pre>
                  ) : (
                    <span className="text-xs text-secondary italic">No payload</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
