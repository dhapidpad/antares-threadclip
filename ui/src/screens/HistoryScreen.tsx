import React from 'react';
import { Film, Calendar, CheckCircle2, Play, FolderOpen } from 'lucide-react';

export const HistoryScreen: React.FC = () => {
  const historyItems = [
    {
      id: 'h1',
      title: 'Shorts_Antares_AI_Intro.mp4',
      preset: 'shorts-1080x1920',
      duration: '45s (1350 frames)',
      date: '19/11/2026 11:45',
      appVersion: 'v3.0.0',
      planHash: 'sha256:7f83b1657ff1...a210',
    },
    {
      id: 'h2',
      title: 'Top_Comments_Review_Threads.mp4',
      preset: 'shorts-1080x1920',
      duration: '30s (900 frames)',
      date: '18/11/2026 16:20',
      appVersion: 'v3.0.0',
      planHash: 'sha256:9a32c451b680...c890',
    },
  ];

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight text-slate-100">
          Lịch sử Xuất bản (S09)
        </h2>
        <p className="text-sm text-slate-400">
          Danh mục video đã xuất kèm siêu dữ liệu kiểm định (app_version, plan_hash, ngày giờ).
        </p>
      </div>

      <div className="space-y-3">
        {historyItems.map((item) => (
          <div
            key={item.id}
            className="bg-slate-900 border border-slate-800 rounded-xl p-4 flex items-center justify-between"
          >
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-lg bg-emerald-500/10 text-emerald-400 flex items-center justify-center">
                <Film className="w-5 h-5" />
              </div>
              <div>
                <h4 className="font-semibold text-xs text-slate-100">{item.title}</h4>
                <div className="flex items-center gap-3 text-[11px] text-slate-400 mt-1">
                  <span className="flex items-center gap-1">
                    <Calendar className="w-3 h-3" />
                    {item.date}
                  </span>
                  <span>• {item.duration}</span>
                  <span className="text-emerald-400 font-mono text-[10px] bg-emerald-500/10 px-1.5 py-0.5 rounded">
                    {item.planHash}
                  </span>
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <button className="p-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-medium flex items-center gap-1 transition">
                <Play className="w-3.5 h-3.5" />
                Mở xem
              </button>
              <button className="p-2 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg text-xs font-medium transition">
                <FolderOpen className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
