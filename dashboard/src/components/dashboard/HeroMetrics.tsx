import { Activity, Server, ShieldAlert, Cpu } from 'lucide-react';
import { Area, AreaChart, ResponsiveContainer } from 'recharts';
import { useSrishti } from '../../contexts/SrishtiContext';

const sparklineData = [
  { value: 10 }, { value: 25 }, { value: 15 }, { value: 40 }, 
  { value: 35 }, { value: 60 }, { value: 45 }, { value: 80 }
];

export function HeroMetrics() {
  const { status, events } = useSrishti();

  const metrics = [
    {
      label: 'Active Agents',
      value: status?.running_agents || 0,
      trend: `${status?.running_agents || 0} online`,
      trendUp: true,
      icon: <Activity className="w-5 h-5 text-accent-teal" />,
      color: 'teal'
    },
    {
      label: 'Cluster Nodes',
      value: status?.cluster_nodes || 1,
      trend: 'Quorum: Healthy',
      trendUp: true,
      icon: <Server className="w-5 h-5 text-accent-indigo" />,
      color: 'indigo'
    },
    {
      label: 'Pending Approvals',
      value: status?.pending_approvals || 0,
      trend: status?.pending_approvals > 0 ? 'Requires Action' : 'All Clear',
      trendUp: status?.pending_approvals === 0,
      icon: <ShieldAlert className="w-5 h-5 text-yellow-400" />,
      color: 'yellow'
    },
    {
      label: 'Total Events',
      value: events.length,
      trend: 'Live Stream Active',
      trendUp: true,
      icon: <Cpu className="w-5 h-5 text-green-400" />,
      color: 'green'
    }
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      {metrics.map((metric, idx) => (
        <div key={idx} className="glass-panel p-6 rounded-xl relative overflow-hidden group">
          <div className="flex justify-between items-start mb-4 relative z-10">
            <div>
              <p className="text-sm text-secondary mb-1">{metric.label}</p>
              <h3 className={`text-3xl font-bold neon-text-${metric.color}`}>{metric.value}</h3>
            </div>
            <div className="p-2 bg-surface rounded-lg border border-border">
              {metric.icon}
            </div>
          </div>
          
          <div className="flex items-center text-sm relative z-10">
            <span className={metric.trendUp ? "text-accent-teal" : "text-yellow-400"}>
              {metric.trend}
            </span>
          </div>

          <div className="absolute -bottom-4 left-0 right-0 h-16 opacity-30 group-hover:opacity-60 transition-opacity">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={sparklineData}>
                <defs>
                  <linearGradient id={`gradient-${idx}`} x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor={metric.trendUp ? "#00d9ff" : "#facc15"} stopOpacity={0.8}/>
                    <stop offset="95%" stopColor={metric.trendUp ? "#00d9ff" : "#facc15"} stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <Area 
                  type="monotone" 
                  dataKey="value" 
                  stroke={metric.trendUp ? "#00d9ff" : "#facc15"} 
                  fillOpacity={1} 
                  fill={`url(#gradient-${idx})`} 
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      ))}
    </div>
  );
}
