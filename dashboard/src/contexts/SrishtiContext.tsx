import { createContext, useContext, useEffect, useState } from 'react';
import type { ReactNode } from 'react';

interface Agent {
  pid: string;
  name: string;
  version: string;
  permissions: string[];
  role: string;
  status: string;
  node: string;
  cpu_usage: string;
  memory_usage: string;
  token_usage: string;
  started_at: string;
}

interface SrishtiEvent {
  name: string;
  source_agent: string;
  target_agent?: string | null;
  payload: any;
  timestamp: string;
}

interface SrishtiContextType {
  status: any;
  agents: Agent[];
  events: SrishtiEvent[];
  approvals: any[];
  cluster: any;
  memoryStats: any[];
  refreshApprovals: () => void;
}

const SrishtiContext = createContext<SrishtiContextType | undefined>(undefined);

export const SrishtiProvider = ({ children }: { children: ReactNode }) => {
  const [status, setStatus] = useState<any>({});
  const [agents, setAgents] = useState<Agent[]>([]);
  const [events, setEvents] = useState<SrishtiEvent[]>([]);
  const [approvals, setApprovals] = useState<any[]>([]);
  const [cluster, setCluster] = useState<any>({});
  const [memoryStats, setMemoryStats] = useState<any[]>([]);

  const fetchData = async () => {
    try {
      const [statusRes, agentsRes, clusterRes, approvalsRes, memoryRes] = await Promise.all([
        fetch('http://localhost:3000/api/status'),
        fetch('http://localhost:3000/api/agents'),
        fetch('http://localhost:3000/api/cluster'),
        fetch('http://localhost:3000/api/approvals'),
        fetch('http://localhost:3000/api/memory'),
      ]);

      setStatus(await statusRes.json());
      const agentData = await agentsRes.json();
      setAgents(agentData.agents);
      setCluster(await clusterRes.json());
      const approvalData = await approvalsRes.json();
      setApprovals(approvalData.pending);
      const memoryData = await memoryRes.json();
      setMemoryStats(memoryData.stats);
    } catch (e) {
      console.error("Failed to fetch Srishti state:", e);
    }
  };

  useEffect(() => {
    fetchData();

    const eventSource = new EventSource('http://localhost:3000/api/events/stream');
    
    eventSource.onmessage = (e) => {
      try {
        const newEvent = JSON.parse(e.data);
        newEvent.timestamp = new Date().toISOString();
        console.log("[FRONTEND] Event received", newEvent);
        setEvents((prev) => [newEvent, ...prev].slice(0, 50));
        
        // Re-fetch state on specific events
        if (
          newEvent.name === 'AgentStarted' || 
          newEvent.name === 'AgentSuspended' || 
          newEvent.name === 'AgentStopped' ||
          newEvent.name === 'ApprovalGranted' ||
          newEvent.name === 'ApprovalRequested' ||
          newEvent.name === 'MemoryStored'
        ) {
          console.log("[FRONTEND] State updated");
          fetchData();
        }
      } catch (err) {
        console.error("Error parsing SSE event:", err);
      }
    };

    return () => {
      eventSource.close();
    };
  }, []);

  return (
    <SrishtiContext.Provider value={{ status, agents, events, approvals, cluster, memoryStats, refreshApprovals: fetchData }}>
      {children}
    </SrishtiContext.Provider>
  );
};

export const useSrishti = () => {
  const context = useContext(SrishtiContext);
  if (context === undefined) {
    throw new Error('useSrishti must be used within a SrishtiProvider');
  }
  return context;
};
