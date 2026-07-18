import { GitMerge, Play, CheckCircle2, AlertCircle, Clock, Zap } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function WorkflowVisualizer() {
  const { agents } = useSrishti();

  const getStatus = (name: string) => {
    const agent = agents.find(a => a.name === name);
    return agent ? agent.status : 'Idle';
  };

  const routerStatus = getStatus("RouterAgent");
  const refundStatus = getStatus("RefundAgent");
  const escalationStatus = getStatus("EscalationAgent");

  const getIcon = (status: string) => {
    if (status === 'Running' || status === 'AgentStarted') return <Zap className="w-8 h-8 text-accent-teal" />;
    if (status === 'Suspended') return <Clock className="w-8 h-8 text-yellow-400 animate-pulse" />;
    if (status === 'Done') return <CheckCircle2 className="w-8 h-8 text-accent-teal" />;
    return <Play className="w-8 h-8 text-secondary" />;
  };

  const getBorderColor = (status: string) => {
    if (status === 'Running' || status === 'AgentStarted' || status === 'Done') return 'border-accent-teal';
    if (status === 'Suspended') return 'border-yellow-400';
    return 'border-border';
  };
  return (
    <div className="space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <GitMerge className="w-8 h-8 text-accent-indigo" />
        <h2 className="text-3xl font-bold text-primary">Workflow Visualizer</h2>
      </div>

      <div className="glass-panel p-8 rounded-xl relative overflow-hidden">
        <h3 className="text-xl font-bold text-primary mb-8">Customer Support Flow</h3>

        <div className="flex items-center justify-between relative max-w-4xl mx-auto">
          {/* Connecting Line */}
          <div className="absolute top-1/2 left-0 right-0 h-1 bg-surface border-y border-border -z-10 -translate-y-1/2"></div>
          
          {/* Animated Flow Line (Router to Refund) */}
          <div className="absolute top-1/2 left-0 w-1/2 h-1 bg-accent-teal -z-10 -translate-y-1/2 shadow-glow-teal animate-pulse"></div>

          {/* RouterAgent Node */}
          <div className="flex flex-col items-center group">
            <div className={`w-16 h-16 rounded-2xl bg-surface border-2 ${getBorderColor(routerStatus)} flex items-center justify-center mb-4 relative`}>
              {getIcon(routerStatus)}
            </div>
            <h4 className="font-bold text-primary">RouterAgent</h4>
            <p className="text-xs text-secondary mt-1">Status: {routerStatus}</p>
            <div className="mt-4 p-3 bg-surface border border-border rounded-lg text-xs font-mono text-secondary max-w-[200px] text-center opacity-0 group-hover:opacity-100 transition-opacity">
              Emit: route_to_refund("CUST-9921")
            </div>
          </div>

          {/* RefundAgent Node */}
          <div className="flex flex-col items-center group">
            <div className={`w-16 h-16 rounded-2xl bg-surface border-2 ${getBorderColor(refundStatus)} flex items-center justify-center mb-4 relative`}>
              {getIcon(refundStatus)}
            </div>
            <h4 className="font-bold text-primary">RefundAgent</h4>
            <p className={`text-xs mt-1 font-bold ${refundStatus === 'Suspended' ? 'text-yellow-400 animate-pulse' : 'text-secondary'}`}>Status: {refundStatus}</p>
            <div className="mt-4 p-3 bg-rose-500/10 border border-rose-500/30 rounded-lg text-xs text-rose-200 max-w-[200px] text-center opacity-0 group-hover:opacity-100 transition-opacity font-mono">
              PolicyViolation: FinancialTransactionLimit(500)
            </div>
          </div>

          {/* EscalationAgent Node */}
          <div className="flex flex-col items-center group">
            <div className={`w-16 h-16 rounded-2xl bg-surface border-2 ${getBorderColor(escalationStatus)} flex items-center justify-center mb-4`}>
              {getIcon(escalationStatus)}
            </div>
            <h4 className="font-bold text-primary">EscalationAgent</h4>
            <p className="text-xs text-secondary mt-1">Status: {escalationStatus}</p>
            <div className="mt-4 p-3 bg-surface border border-border rounded-lg text-xs font-mono text-secondary max-w-[200px] text-center opacity-0 group-hover:opacity-100 transition-opacity">
              Waiting for `escalate_to_human`
            </div>
          </div>
        </div>

      </div>
    </div>
  );
}
