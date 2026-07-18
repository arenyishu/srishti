import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { useSrishti } from '../../contexts/SrishtiContext';

export function AgentActivity() {
  const { events } = useSrishti();

  // In a real app we'd bin events by time.
  // For the prototype, we just show a static or mock distribution if no events.
  const chartData = events.length > 0 ? 
    [{ time: 'Now', tokens: events.length * 10, calls: events.length }] :
    [
      { time: '00:00', tokens: 1200, calls: 45 },
      { time: '04:00', tokens: 1800, calls: 62 },
      { time: '08:00', tokens: 4500, calls: 120 },
      { time: '12:00', tokens: 8200, calls: 240 },
      { time: '16:00', tokens: 6100, calls: 185 },
      { time: '20:00', tokens: 3200, calls: 95 },
      { time: '24:00', tokens: 2100, calls: 60 }
    ];

  return (
    <div className="glass-panel rounded-xl p-6 h-full">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h3 className="text-xl font-bold text-primary">Global Activity</h3>
          <p className="text-sm text-secondary">Token usage vs RPC Calls</p>
        </div>
        <select className="bg-surface border border-border text-primary text-sm rounded-lg px-3 py-2 focus:ring-accent-teal focus:border-accent-teal">
          <option>Last 24 Hours</option>
          <option>Last 7 Days</option>
          <option>Last 30 Days</option>
        </select>
      </div>

      <div className="h-[300px] w-full mt-4 relative z-10">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={chartData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
            <defs>
              <linearGradient id="colorTokens" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#00d9ff" stopOpacity={0.4}/>
                <stop offset="95%" stopColor="#00d9ff" stopOpacity={0}/>
              </linearGradient>
              <linearGradient id="colorCalls" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#4f46e5" stopOpacity={0.4}/>
                <stop offset="95%" stopColor="#4f46e5" stopOpacity={0}/>
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
            <XAxis dataKey="time" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
            <YAxis yAxisId="left" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} tickFormatter={(value) => `${Number(value) / 1000}k`} />
            <YAxis yAxisId="right" orientation="right" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
            <Tooltip 
              contentStyle={{ backgroundColor: '#111827', borderColor: '#1e293b', borderRadius: '0.5rem' }}
              itemStyle={{ color: '#ffffff' }}
            />
            <Area yAxisId="left" type="monotone" dataKey="tokens" stroke="#00d9ff" strokeWidth={2} fillOpacity={1} fill="url(#colorTokens)" />
            <Area yAxisId="right" type="monotone" dataKey="calls" stroke="#4f46e5" strokeWidth={2} fillOpacity={1} fill="url(#colorCalls)" />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
