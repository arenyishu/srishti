import { ShieldCheck, ShieldAlert, Lock, Activity } from 'lucide-react';
import { useSrishti } from '../../contexts/SrishtiContext';

export function PolicyCenter() {
  const { status } = useSrishti();

  return (
    <div className="space-y-6">
      <div className="flex items-center space-x-3 mb-6">
        <ShieldCheck className="w-8 h-8 text-accent-teal" />
        <h2 className="text-3xl font-bold text-primary">Policy & Security Center</h2>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="glass-panel p-6 rounded-xl flex items-center space-x-4 border-b-2 border-b-accent-teal">
          <div className="p-3 bg-surface rounded-lg">
            <Lock className="w-6 h-6 text-accent-teal" />
          </div>
          <div>
            <h4 className="text-secondary text-sm">Active OS Policies</h4>
            <p className="text-2xl font-bold text-primary">{status?.active_policies || 1}</p>
          </div>
        </div>
        <div className="glass-panel p-6 rounded-xl flex items-center space-x-4 border-b-2 border-b-yellow-400">
          <div className="p-3 bg-surface rounded-lg">
            <ShieldAlert className="w-6 h-6 text-yellow-400" />
          </div>
          <div>
            <h4 className="text-secondary text-sm">Violations (24h)</h4>
            <p className="text-2xl font-bold text-primary">0</p>
          </div>
        </div>
        <div className="glass-panel p-6 rounded-xl flex items-center space-x-4 border-b-2 border-b-accent-indigo">
          <div className="p-3 bg-surface rounded-lg">
            <Activity className="w-6 h-6 text-accent-indigo" />
          </div>
          <div>
            <h4 className="text-secondary text-sm">Auth Events</h4>
            <p className="text-2xl font-bold text-primary">124</p>
          </div>
        </div>
      </div>

      <h3 className="text-xl font-bold text-primary mb-4">Enforced Kernel Policies</h3>
      <div className="glass-panel rounded-xl overflow-hidden">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-surface/50 text-secondary text-sm">
              <th className="p-4 font-medium border-b border-border">POLICY NAME</th>
              <th className="p-4 font-medium border-b border-border">SCOPE</th>
              <th className="p-4 font-medium border-b border-border">ACTION</th>
              <th className="p-4 font-medium border-b border-border">STATUS</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border/50">
            <tr className="hover:bg-surface/30 transition-colors">
              <td className="p-4 font-medium text-primary">FinancialTransactionLimit</td>
              <td className="p-4 text-sm text-secondary">Global</td>
              <td className="p-4 text-sm text-yellow-400">Suspend For Approval (&gt;$100)</td>
              <td className="p-4"><span className="px-2 py-1 bg-accent-teal/10 text-accent-teal text-xs rounded border border-accent-teal/20">Enforced</span></td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );
}
