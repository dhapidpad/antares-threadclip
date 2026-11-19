import React, { useState } from 'react';
import { Clapperboard, FolderOpen, Play, CheckCircle2 } from 'lucide-react';

export const RenderQueueScreen: React.FC = () => {
  const [selectedPreset, setSelectedPreset] = useState('shorts-1080x1920');
  const [rendering, setRendering] = useState(false);
  const [progress, setProgress] = useState(100);
  const [jobDone, setJobDone] = useState(true);

  const startRender = () => {
    setRendering(true);
    setProgress(0);
    setJobDone(false);

    const timer = setInterval(() => {
      setProgress((p) => {
        if (p >= 100) {
          clearInterval(timer);
          setRendering(false);
          setJobDone(true);
          return 100;
        }
        return p + 25;
      });
    }, 400);
  };

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight text-slate-100">
          Hàng đợi Kết xuất Video (S08)
        </h2>
        <p className="text-sm text-slate-400">
          Xuất video MP4 H.264 + AAC frame-accurate và deterministic qua tiến trình FFmpeg sidecar.
        </p>
      </div>

      {/* Preset Selection */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {[
          {
            id: 'shorts-1080x1920',
            name: 'Shorts Chuẩn 1080x1920',
            desc: 'CRF 18, High Profile, 30fps, AAC 192k, +faststart',
            badge: 'Mặc định',
          },
          {
            id: 'shorts-720x1280',
            name: 'Shorts 720x1280',
            desc: 'CRF 20, Fast speed, 30fps, AAC 128k',
            badge: 'Nhẹ',
          },
          {
            id: 'draft-540x960',
            name: 'Bản nháp Draft 540x960',
            desc: 'CRF 26, Veryfast, 30fps, AAC 96k',
            badge: 'Siêu nhanh',
          },
        ].map((preset) => (
          <div
            key={preset.id}
            onClick={() => setSelectedPreset(preset.id)}
            className={`p-4 rounded-xl border cursor-pointer transition flex flex-col justify-between ${
              selectedPreset === preset.id
                ? 'bg-slate-900 border-emerald-500 shadow-sm'
                : 'bg-slate-900/60 border-slate-800 hover:border-slate-700'
            }`}
          >
            <div>
              <div className="flex items-center justify-between mb-1">
                <span className="font-semibold text-xs text-slate-100">{preset.name}</span>
                <span className="text-[10px] bg-slate-800 text-slate-300 px-2 py-0.5 rounded">
                  {preset.badge}
                </span>
              </div>
              <p className="text-[11px] text-slate-400">{preset.desc}</p>
            </div>
          </div>
        ))}
      </div>

      {/* Active Job Status */}
      <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-emerald-500/10 text-emerald-400 flex items-center justify-center">
              <Clapperboard className="w-5 h-5" />
            </div>
            <div>
              <h4 className="font-semibold text-xs text-slate-100">
                short_ai_antares_sample.mp4
              </h4>
              <p className="text-[11px] text-slate-400 font-mono">
                plan_hash: 7a8f9c2d1e0b... • Preset: {selectedPreset}
              </p>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={startRender}
              disabled={rendering}
              className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white rounded-lg text-xs font-medium flex items-center gap-1.5 shadow-sm transition"
            >
              <Play className="w-3.5 h-3.5" />
              {rendering ? 'Đang Render...' : 'Bắt đầu Render'}
            </button>
            <button className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg text-xs font-medium flex items-center gap-1.5 transition">
              <FolderOpen className="w-3.5 h-3.5" />
              Mở thư mục
            </button>
          </div>
        </div>

        {/* Progress */}
        <div className="space-y-1.5">
          <div className="flex justify-between text-xs text-slate-400">
            <span>Tiến độ kết xuất</span>
            <span className="font-mono text-emerald-400">{progress}%</span>
          </div>
          <div className="w-full bg-slate-950 h-2 rounded-full overflow-hidden border border-slate-800">
            <div
              style={{ width: `${progress}%` }}
              className="bg-emerald-500 h-full transition-all duration-300"
            ></div>
          </div>
        </div>

        {jobDone && (
          <div className="bg-emerald-500/10 border border-emerald-500/20 rounded-lg p-3 flex items-center gap-2 text-xs text-emerald-400">
            <CheckCircle2 className="w-4 h-4 flex-shrink-0" />
            <span>Kết xuất hoàn tất: Video đạt chuẩn frame-accurate và trùng khớp kế hoạch plan_hash!</span>
          </div>
        )}
      </div>
    </div>
  );
};
