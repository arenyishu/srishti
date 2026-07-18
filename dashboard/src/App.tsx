
import { useState } from 'react';
import { DashboardLayout } from './components/layout/DashboardLayout';
import { HeroMetrics } from './components/dashboard/HeroMetrics';
import { AgentActivity } from './components/dashboard/AgentActivity';
import { AgentStatusTable } from './components/dashboard/AgentStatusTable';
import { MemoryPanel } from './components/dashboard/MemoryPanel';
import { RecentEvents } from './components/dashboard/RecentEvents';

import { AgentExplorer } from './components/views/AgentExplorer';
import { PolicyCenter } from './components/views/PolicyCenter';
import { ApprovalCenter } from './components/views/ApprovalCenter';
import { AuditCenter } from './components/views/AuditCenter';
import { WorkflowVisualizer } from './components/views/WorkflowVisualizer';
import { DebugPage } from './components/views/DebugPage';

function DashboardOverview() {
  return (
    <>
      <div className="flex justify-between items-end mb-8">
        <div>
          <h1 className="text-3xl font-bold text-primary neon-text tracking-wide mb-2">Overview</h1>
          <p className="text-secondary">Monitor and manage your Srishti distributed agents.</p>
        </div>
        <div className="flex space-x-3">
          <button className="px-4 py-2 bg-surface border border-border hover:border-accent-indigo text-primary rounded-lg transition-colors">
            Manage Cluster
          </button>
          <button className="px-4 py-2 bg-accent-indigo hover:bg-accent-indigo/80 text-white shadow-glow-indigo rounded-lg transition-all">
            Deploy Agent
          </button>
        </div>
      </div>

      <HeroMetrics />
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
        <div className="lg:col-span-2">
          <AgentActivity />
        </div>
        <div className="lg:col-span-1">
          <MemoryPanel />
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2">
          <AgentStatusTable />
        </div>
        <div className="lg:col-span-1">
          <RecentEvents />
        </div>
      </div>
    </>
  );
}

function App() {
  const [activeView, setActiveView] = useState('Overview');

  return (
    <DashboardLayout activeView={activeView} onNavigate={setActiveView}>
      <div className="max-w-7xl mx-auto space-y-6">
        {activeView === 'Overview' && <DashboardOverview />}
        {activeView === 'Agents' && <AgentExplorer />}
        {activeView === 'Workflows' && <WorkflowVisualizer />}
        {activeView === 'Policies' && <PolicyCenter />}
        {activeView === 'Approvals' && <ApprovalCenter />}
        {activeView === 'Audit Logs' && <AuditCenter />}
        {activeView === 'Debug' && <DebugPage />}
        {/* Memory and Cluster can route here too, but for MVP we show placeholders or just map them */}
        {(activeView === 'Memory' || activeView === 'Cluster' || activeView === 'Metrics' || activeView === 'Packages' || activeView === 'Deployments' || activeView === 'Marketplace' || activeView === 'Settings') && (
          <div className="glass-panel p-8 text-center text-secondary rounded-xl">
            {activeView} Center coming in v1.1.
          </div>
        )}
      </div>
    </DashboardLayout>
  );
}

export default App;
