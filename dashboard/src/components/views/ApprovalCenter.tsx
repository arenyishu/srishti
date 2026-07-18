import { CheckSquare, Check, X } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function ApprovalCenter() {
  const { approvals, refreshApprovals } = useSrishti();

  const handleApprove = async (id: string) => {
    await fetch(`http://localhost:3000/api/approvals/${id}/approve`, { method: 'POST' });
    refreshApprovals();
  };

  const handleReject = async (id: string) => {
    await fetch(`http://localhost:3000/api/approvals/${id}/reject`, { method: 'POST' });
    refreshApprovals();
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <CheckSquare className="w-8 h-8 text-accent-indigo" />
        <h2 className="text-3xl font-bold text-primary">Approval Center</h2>
      </div>

      <div className="grid grid-cols-1 gap-4">
        {approvals.length === 0 ? (
          <div className="glass-panel p-8 text-center text-secondary rounded-xl">
            No pending approvals. All agents operating autonomously.
          </div>
        ) : (
          approvals.map((approval: any) => (
            <div key={approval.id} className="glass-panel p-6 rounded-xl flex items-center justify-between border-l-4 border-l-yellow-400">
              <div>
                <div className="flex items-center space-x-2 mb-2">
                  <span className="bg-yellow-400/20 text-yellow-400 px-2 py-1 rounded text-xs font-bold uppercase">Requires Action</span>
                  <span className="text-sm text-secondary">Agent: {approval.agent_pid}</span>
                </div>
                <h4 className="text-lg font-bold text-primary mb-1">Intent: {approval.action}</h4>
                <p className="text-sm text-secondary">{approval.context}</p>
                {approval.amount && <p className="text-sm text-accent-teal mt-1">Amount: ${approval.amount}</p>}
              </div>
              <div className="flex space-x-3">
                <button 
                  onClick={() => handleReject(approval.id)}
                  className="flex items-center space-x-2 px-4 py-2 bg-surface border border-border hover:border-rose-400 text-rose-400 rounded-lg transition-colors"
                >
                  <X className="w-4 h-4" />
                  <span>Reject</span>
                </button>
                <button 
                  onClick={() => handleApprove(approval.id)}
                  className="flex items-center space-x-2 px-4 py-2 bg-accent-teal hover:bg-accent-teal/80 text-background font-medium rounded-lg transition-colors shadow-glow-teal"
                >
                  <Check className="w-4 h-4" />
                  <span>Approve & Resume</span>
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
