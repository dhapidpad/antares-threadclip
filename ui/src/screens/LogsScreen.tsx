import React, { useState } from 'react';
import { Terminal, Shield, Download, RefreshCw, Check } from 'lucide-react';

export const LogsScreen: React.FC = () => {
  const [copied, setCopied] = useState(false);

  const logs = [
    { time: '11:45:02', level: 'INFO', msg: 'Antares ThreadClip Studio v3.0.0 initialized on windows.' },
    { time: '11:45:03', level: 'INFO', msg: 'SQLite connection opened in WAL mode with FTS5 enabled.' },
    { time: '11:45:10', level: 'INFO', msg: 'Threads import completed: 20 posts, 420 replies loaded.' },
    { time: '11:46:00', level: 'DEBUG', msg: 'OAuth token validated via keychain: [REDACTED_TOKEN]' },
    { time: '11:46:15', level: 'INFO', msg: 'TTS audio generated for "Card #1", cached as hash 7a8f9c2d1e0b.wav' },
    { time: '11:47:00', level: 'INFO', msg: 'RenderPlan created with plan_hash sha256:7f83b165... Duration: 45s.' },
  ];

  const handleExport = () => {
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight text-slate-100">
            Nhật ký & Chẩn đoán (S11)
          </h2>
          <p className="text-sm text-slate-400">
            Quan sát trạng thái hệ thống. Tự động che thông tin nhạy cảm (token, email).
          </p>
        </div>

        <button
          onClick={handleExport}
          className="flex items-center gap-2 bg-slate-800 hover:bg-slate-700 text-slate-200 px-4 py-2 rounded-lg text-xs font-medium transition shadow-sm"
        >
          {copied ? <Check className="w-4 h-4 text-emerald-400" /> : <Download className="w-4 h-4" />}
          {copied ? 'Đã xuất gói chẩn đoán' : 'Xuất gói chẩn đoán'}
        </button>
      </div>

      <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden flex flex-col h-[520px]">
        <div className="p-3 border-b border-slate-800 bg-slate-900/80 flex items-center justify-between text-xs">
          <div className="flex items-center gap-2">
            <Terminal className="w-4 h-4 text-emerald-400" />
            <span className="font-semibold text-slate-200">System Trace Log (Redacted)</span>
          </div>
          <span className="bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded text-[10px] font-medium flex items-center gap-1">
            <Shield className="w-3 h-3" />
            Redaction Active
          </span>
        </div>

        <div className="flex-1 p-4 font-mono text-xs overflow-y-auto space-y-1.5 bg-slate-950">
          {logs.map((log, i) => (
            <div key={i} className="flex items-start gap-2.5 leading-relaxed">
              <span className="text-slate-500 select-none">{log.time}</span>
              <span
                className={`font-semibold text-[10px] px-1.5 py-0.2 rounded select-none ${
                  log.level === 'INFO'
                    ? 'bg-blue-500/10 text-blue-400'
                    : 'bg-slate-800 text-slate-400'
                }`}
              >
                {log.level}
              </span>
              <span className="text-slate-300">{log.msg}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
