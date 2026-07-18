import { Database } from 'lucide-react';
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip } from 'recharts';
import { useSrishti } from '../../contexts/SrishtiContext';

const COLORS = ['#00d9ff', '#4f46e5', '#8b5cf6', '#1e293b'];

export function MemoryPanel() {
  const { memoryStats } = useSrishti();

  const totalMem = memoryStats ? memoryStats.reduce((acc, curr) => acc + curr.entries, 0) : 0;

  const mockMemData = memoryStats && memoryStats.length > 0 
    ? memoryStats.map((stat) => ({
        name: stat.collection,
        value: stat.entries,
      }))
    : [
        { name: 'Waiting for memory...', value: 1 }
      ];

  return (
    <div className="glass-panel rounded-xl p-6 h-full flex flex-col">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h3 className="text-xl font-bold text-primary">Memory Distribution</h3>
          <p className="text-sm text-secondary">Total Process Memory</p>
        </div>
        <div className="p-2 bg-surface rounded-lg border border-border">
          <Database className="w-5 h-5 text-accent-teal" />
        </div>
      </div>

      <div className="flex-1 min-h-[200px] relative">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={mockMemData}
              cx="50%"
              cy="50%"
              innerRadius={60}
              outerRadius={80}
              paddingAngle={5}
              dataKey="value"
              stroke="none"
            >
              {mockMemData.map((_entry, index) => (
                <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
              ))}
            </Pie>
            <Tooltip 
              contentStyle={{ backgroundColor: '#111827', borderColor: '#1e293b', borderRadius: '0.5rem' }}
              itemStyle={{ color: '#ffffff' }}
            />
          </PieChart>
        </ResponsiveContainer>
        <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
          <span className="text-2xl font-bold text-primary">{totalMem}</span>
          <span className="text-xs text-secondary">Entries</span>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-2 gap-4">
        {mockMemData.map((item, idx) => (
          <div key={idx} className="flex items-center space-x-2">
            <div className="w-3 h-3 rounded-full" style={{ backgroundColor: COLORS[idx] }} />
            <div>
              <p className="text-xs text-secondary">{item.name}</p>
              <p className="text-sm font-bold text-primary">{item.value} Entries</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
