import React from 'react';
import { Database, ShieldCheck, CheckCircle2, Clock } from 'lucide-react';

interface StatusBarProps {
  currentProject: string | null;
  saveStatus: string;
}

export const StatusBar: React.FC<StatusBarProps> = ({ currentProject, saveStatus }) => {
  return (
    <footer className="h-7 bg-slate-900 border-t border-slate-800 px-4 flex items-center justify-between text-[11px] text-slate-400 select-none">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5 text-emerald-400 font-medium">
          <ShieldCheck className="w-3.5 h-3.5" />
          Local-first Active
        </span>
        <span className="flex items-center gap-1 text-slate-300">
          <Database className="w-3.5 h-3.5 text-blue-400" />
          SQLite WAL • FTS5
        </span>
        {currentProject && (
          <span className="text-slate-300">
            Dự án: <strong className="text-slate-100">{currentProject}</strong>
          </span>
        )}
      </div>

      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1 text-slate-400">
          <Clock className="w-3.5 h-3.5 text-amber-400" />
          Hạn OAuth: 58 ngày
        </span>
        <span className="flex items-center gap-1 text-emerald-400">
          <CheckCircle2 className="w-3.5 h-3.5" />
          {saveStatus}
        </span>
      </div>
    </footer>
  );
};
