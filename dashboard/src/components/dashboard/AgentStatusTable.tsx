import { MoreVertical, CheckCircle2, XCircle, PlayCircle, PauseCircle } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function AgentStatusTable() {
  const { agents } = useSrishti();

  const getStatusIcon = (status: string) => {
    switch(status.toLowerCase()) {
      case 'running': return <PlayCircle className="w-4 h-4 text-accent-teal" />;
      case 'suspended': return <PauseCircle className="w-4 h-4 text-yellow-400" />;
      case 'error': return <XCircle className="w-4 h-4 text-rose-400" />;
      default: return <CheckCircle2 className="w-4 h-4 text-secondary" />;
    }
  };

  return (
    <div className="glass-panel rounded-xl overflow-hidden">
      <div className="p-6 border-b border-border flex justify-between items-center">
        <h3 className="text-xl font-bold text-primary">Active Agents</h3>
        <button className="text-sm text-accent-teal hover:text-accent-teal/80 transition-colors">
          View All
        </button>
      </div>
      
      <div className="overflow-x-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-surface/50 text-secondary text-sm">
              <th className="p-4 font-medium border-b border-border">AGENT</th>
              <th className="p-4 font-medium border-b border-border">STATUS</th>
              <th className="p-4 font-medium border-b border-border">CPU</th>
              <th className="p-4 font-medium border-b border-border">MEMORY</th>
              <th className="p-4 font-medium border-b border-border">UPTIME</th>
              <th className="p-4 font-medium border-b border-border"></th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border/50">
            {agents.length === 0 ? (
              <tr>
                <td colSpan={6} className="p-8 text-center text-secondary">
                  No agents currently running.
                </td>
              </tr>
            ) : agents.map((agent) => (
              <tr key={agent.pid} className="hover:bg-surface/30 transition-colors group">
                <td className="p-4">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 rounded bg-surface border border-border flex items-center justify-center font-bold text-accent-indigo">
                      {agent.name.charAt(0)}
                    </div>
                    <div>
                      <p className="font-medium text-primary group-hover:text-accent-teal transition-colors">
                        {agent.name}
                      </p>
                      <p className="text-xs text-secondary">{agent.pid}</p>
                    </div>
                  </div>
                </td>
                <td className="p-4">
                  <div className="flex items-center space-x-2">
                    {getStatusIcon(agent.status)}
                    <span className="text-sm capitalize">{agent.status}</span>
                  </div>
                </td>
                <td className="p-4 text-sm text-secondary">{agent.cpu_usage}</td>
                <td className="p-4 text-sm text-secondary">{agent.memory_usage}</td>
                <td className="p-4 text-sm text-secondary">{agent.started_at}</td>
                <td className="p-4 text-right">
                  <button className="p-2 hover:bg-surface rounded-lg text-secondary hover:text-primary transition-colors">
                    <MoreVertical className="w-4 h-4" />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
