import { Activity, Server, Network, Database } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function DebugPage() {
  const { status, agents, events, cluster, memoryStats } = useSrishti();

  return (
    <div className="space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <Activity className="w-8 h-8 text-rose-400" />
        <h2 className="text-3xl font-bold text-primary">Debug & Telemetry</h2>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass-panel p-6 rounded-xl">
          <div className="flex items-center space-x-2 mb-4 border-b border-border pb-4">
            <Server className="w-5 h-5 text-accent-indigo" />
            <h3 className="text-xl font-bold text-primary">API Health</h3>
          </div>
          <pre className="text-sm font-mono text-secondary overflow-x-auto">
            {JSON.stringify(status, null, 2)}
          </pre>
        </div>

        <div className="glass-panel p-6 rounded-xl">
          <div className="flex items-center space-x-2 mb-4 border-b border-border pb-4">
            <Network className="w-5 h-5 text-accent-teal" />
            <h3 className="text-xl font-bold text-primary">Cluster State</h3>
          </div>
          <pre className="text-sm font-mono text-secondary overflow-x-auto">
            {JSON.stringify(cluster, null, 2)}
          </pre>
        </div>

        <div className="glass-panel p-6 rounded-xl">
          <div className="flex items-center space-x-2 mb-4 border-b border-border pb-4">
            <Database className="w-5 h-5 text-emerald-400" />
            <h3 className="text-xl font-bold text-primary">Memory Telemetry</h3>
          </div>
          <pre className="text-sm font-mono text-secondary overflow-x-auto">
            {JSON.stringify(memoryStats, null, 2)}
          </pre>
        </div>

        <div className="glass-panel p-6 rounded-xl">
          <div className="flex items-center space-x-2 mb-4 border-b border-border pb-4">
            <Activity className="w-5 h-5 text-yellow-400" />
            <h3 className="text-xl font-bold text-primary">Raw Metrics</h3>
          </div>
          <ul className="space-y-2 text-sm font-mono text-secondary">
            <li>Active Agents: <span className="text-primary">{agents.length}</span></li>
            <li>Captured Events: <span className="text-primary">{events.length}</span></li>
            <li>Memory Partitions: <span className="text-primary">{memoryStats ? memoryStats.length : 0}</span></li>
          </ul>
        </div>
      </div>
    </div>
  );
}
