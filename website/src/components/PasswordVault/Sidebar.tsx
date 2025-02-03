import { Plus, Lock, Settings, LogOut } from 'lucide-react';
import Link from 'next/link';

interface SidebarProps {
  onAddPassword: () => void;
}

export function Sidebar({ onAddPassword }: SidebarProps) {
  return (
    <div className="w-16 min-h-screen bg-[#2d0059] border-r border-purple-500/20 flex flex-col items-center py-4">
      <div className="flex-1 flex flex-col items-center gap-4">
        <button
          onClick={onAddPassword}
          className="p-3 rounded-lg hover:bg-purple-500/20 text-purple-300 transition-colors group relative"
          title="Add Password"
        >
          <Plus size={24} />
          <span className="absolute left-full ml-2 px-2 py-1 bg-[#3d007a] rounded text-xs text-white opacity-0 group-hover:opacity-100 pointer-events-none whitespace-nowrap">
            Add Password
          </span>
        </button>
      </div>
      
      <div className="flex flex-col items-center gap-4">
        <button
          className="p-3 rounded-lg hover:bg-purple-500/20 text-purple-300 transition-colors group relative"
          title="Settings"
        >
          <Settings size={24} />
          <span className="absolute left-full ml-2 px-2 py-1 bg-[#3d007a] rounded text-xs text-white opacity-0 group-hover:opacity-100 pointer-events-none">
            Settings
          </span>
        </button>
        <button
          className="p-3 rounded-lg hover:bg-red-500/20 text-red-400 transition-colors group relative"
          title="Logout"
        >
          <LogOut size={24} />
          <span className="absolute left-full ml-2 px-2 py-1 bg-[#3d007a] rounded text-xs text-white opacity-0 group-hover:opacity-100 pointer-events-none">
            Logout
          </span>
        </button>
      </div>
    </div>
  );
} 