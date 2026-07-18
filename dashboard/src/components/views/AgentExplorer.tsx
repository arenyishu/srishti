import { useState } from 'react';
import { Cpu, TerminalSquare, MemoryStick, Activity, Network, X, Shield, Box, Database, AlertTriangle } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function AgentExplorer() {
  const { agents } = useSrishti();
  const [selectedAgentId, setSelectedAgentId] = useState<string | null>(null);

  const selectedAgent = agents.find(a => a.pid === selectedAgentId);

  return (
    <div className="space-y-6 relative">
      <div className="flex items-center space-x-3 mb-6">
        <Cpu className="w-8 h-8 text-accent-indigo" />
        <h2 className="text-3xl font-bold text-primary">Agent Explorer</h2>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
        {agents.length === 0 ? (
          <div className="glass-panel p-8 text-center text-secondary rounded-xl xl:col-span-2">
            No active agents discovered in the process table.
          </div>
        ) : (
          agents.map((agent) => (
            <div key={agent.pid} className="glass-panel p-6 rounded-xl relative overflow-hidden group hover:border-accent-indigo/50 transition-colors">
              <div className="flex justify-between items-start mb-6">
                <div className="flex items-center space-x-4">
                  <div className="w-12 h-12 rounded-lg bg-surface border border-border flex items-center justify-center font-bold text-xl text-accent-indigo shadow-glow-indigo">
                    {agent.name.charAt(0)}
                  </div>
                  <div>
                    <h3 className="text-xl font-bold text-primary">{agent.name}</h3>
                    <p className="text-sm text-secondary font-mono">PID: {agent.pid}</p>
                  </div>
                </div>
                <div className={`px-3 py-1 rounded-full text-xs font-bold uppercase border ${agent.status === 'Running' ? 'bg-accent-teal/10 text-accent-teal border-accent-teal/30' : 'bg-yellow-400/10 text-yellow-400 border-yellow-400/30'}`}>
                  {agent.status}
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4 mb-6">
                <div className="bg-surface/50 p-3 rounded-lg border border-border">
                  <div className="flex items-center space-x-2 mb-1">
                    <Activity className="w-4 h-4 text-secondary" />
                    <span className="text-xs text-secondary uppercase">CPU Usage</span>
                  </div>
                  <p className="font-mono text-primary">{agent.cpu_usage}</p>
                </div>
                <div className="bg-surface/50 p-3 rounded-lg border border-border">
                  <div className="flex items-center space-x-2 mb-1">
                    <MemoryStick className="w-4 h-4 text-secondary" />
                    <span className="text-xs text-secondary uppercase">Memory</span>
                  </div>
                  <p className="font-mono text-primary">{agent.memory_usage}</p>
                </div>
                <div className="bg-surface/50 p-3 rounded-lg border border-border">
                  <div className="flex items-center space-x-2 mb-1">
                    <TerminalSquare className="w-4 h-4 text-secondary" />
                    <span className="text-xs text-secondary uppercase">Tokens</span>
                  </div>
                  <p className="font-mono text-primary">{agent.token_usage}</p>
                </div>
                <div className="bg-surface/50 p-3 rounded-lg border border-border">
                  <div className="flex items-center space-x-2 mb-1">
                    <Network className="w-4 h-4 text-secondary" />
                    <span className="text-xs text-secondary uppercase">Node</span>
                  </div>
                  <p className="font-mono text-primary">{agent.node}</p>
                </div>
              </div>

              <div className="flex justify-end space-x-3">
                <button className="px-4 py-2 text-sm bg-surface border border-border hover:border-rose-400 text-rose-400 rounded-lg transition-colors">
                  Terminate
                </button>
                <button onClick={() => setSelectedAgentId(agent.pid)} className="px-4 py-2 text-sm bg-accent-indigo hover:bg-accent-indigo/80 text-white rounded-lg transition-colors shadow-glow-indigo">
                  View Introspection
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {selectedAgent && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-6">
          <div className="glass-panel w-full max-w-4xl max-h-[85vh] rounded-2xl border border-accent-indigo/30 flex flex-col overflow-hidden shadow-glow-indigo">
            <div className="flex justify-between items-center p-6 border-b border-border bg-surface/80">
              <div className="flex items-center space-x-4">
                <div className="w-10 h-10 rounded-lg bg-surface border border-border flex items-center justify-center font-bold text-lg text-accent-indigo">
                  {selectedAgent.name.charAt(0)}
                </div>
                <div>
                  <h3 className="text-2xl font-bold text-primary">{selectedAgent.name}</h3>
                  <p className="text-sm text-secondary font-mono">{selectedAgent.pid} • v{selectedAgent.version}</p>
                </div>
              </div>
              <button onClick={() => setSelectedAgentId(null)} className="p-2 rounded-lg hover:bg-surface text-secondary hover:text-primary transition-colors">
                <X className="w-6 h-6" />
              </button>
            </div>
            
            <div className="p-6 overflow-y-auto flex-1 custom-scrollbar space-y-6">
              
              <div className="grid grid-cols-2 gap-6">
                <div className="bg-surface/50 p-4 rounded-xl border border-border">
                  <div className="flex items-center space-x-2 mb-3">
                    <Shield className="w-5 h-5 text-accent-teal" />
                    <h4 className="font-bold text-primary">Permissions</h4>
                  </div>
                  <div className="space-y-2">
                    {selectedAgent.permissions && selectedAgent.permissions.length > 0 ? (
                      selectedAgent.permissions.map(perm => (
                        <div key={perm} className="flex items-center justify-between text-sm p-2 rounded bg-surface border border-border">
                          <span className="font-mono text-secondary">{perm}</span>
                          <span className="text-accent-teal font-bold">GRANTED</span>
                        </div>
                      ))
                    ) : (
                      <div className="text-sm p-2 text-secondary">No specific permissions granted.</div>
                    )}
                  </div>
                </div>

                <div className="bg-surface/50 p-4 rounded-xl border border-border">
                  <div className="flex items-center space-x-2 mb-3">
                    <TerminalSquare className="w-5 h-5 text-accent-indigo" />
                    <h4 className="font-bold text-primary">Current Intent</h4>
                  </div>
                  <div className="p-3 bg-surface border border-border rounded font-mono text-sm text-secondary break-all">
                    Achieve "Process refund requests and generate response" using OPENAI_API_KEY
                  </div>
                </div>
              </div>

              <div className="bg-surface/50 p-4 rounded-xl border border-border">
                <div className="flex items-center space-x-2 mb-3">
                  <Database className="w-5 h-5 text-emerald-400" />
                  <h4 className="font-bold text-primary">Memory Partitions</h4>
                </div>
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-border text-secondary text-left">
                      <th className="pb-2">Partition</th>
                      <th className="pb-2">Type</th>
                      <th className="pb-2">Size</th>
                      <th className="pb-2 text-right">Items</th>
                    </tr>
                  </thead>
                  <tbody className="font-mono text-primary">
                    <tr className="border-b border-border/50">
                      <td className="py-2">refund_history</td>
                      <td className="py-2 text-secondary">Vector Store</td>
                      <td className="py-2">2.4 MB</td>
                      <td className="py-2 text-right">142</td>
                    </tr>
                    <tr>
                      <td className="py-2">customer_profile</td>
                      <td className="py-2 text-secondary">Key-Value</td>
                      <td className="py-2">84 KB</td>
                      <td className="py-2 text-right">1</td>
                    </tr>
                  </tbody>
                </table>
              </div>

              <div className="bg-surface/50 p-4 rounded-xl border border-border">
                <div className="flex items-center space-x-2 mb-3">
                  <Box className="w-5 h-5 text-orange-400" />
                  <h4 className="font-bold text-primary">Tool Calls</h4>
                </div>
                <div className="space-y-2 font-mono text-sm">
                  <div className="p-2 border border-border rounded flex justify-between bg-surface">
                    <span className="text-secondary">search_customer_db("CUST-9921")</span>
                    <span className="text-green-400">SUCCESS</span>
                  </div>
                  <div className="p-2 border border-border rounded flex justify-between bg-surface">
                    <span className="text-secondary">stripe.issue_refund(50.0)</span>
                    <span className="text-yellow-400">PENDING_APPROVAL</span>
                  </div>
                </div>
              </div>

              <div className="bg-surface/50 p-4 rounded-xl border border-border border-l-4 border-l-rose-500">
                <div className="flex items-center space-x-2 mb-3">
                  <AlertTriangle className="w-5 h-5 text-rose-500" />
                  <h4 className="font-bold text-primary">Policy Violations</h4>
                </div>
                <div className="p-3 bg-rose-500/10 border border-rose-500/20 rounded text-rose-200 text-sm">
                  <span className="font-bold">FinancialTransactionLimit:</span> Refund amount $500 exceeds local threshold. Requires human-in-the-loop approval.
                </div>
              </div>
              
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
