import React, { useState } from 'react';
import { Play, Pause, Camera, RotateCcw } from 'lucide-react';

export const PreviewScreen: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(35);

  return (
    <div className="flex h-full bg-slate-950 items-center justify-center p-6 gap-8">
      {/* 9:16 Phone Canvas Mockup */}
      <div className="relative w-[340px] h-[604px] bg-black rounded-3xl border-4 border-slate-800 shadow-2xl overflow-hidden flex flex-col justify-between p-4">
        {/* Background Video Simulation */}
        <div className="absolute inset-0 bg-gradient-to-b from-slate-900 via-indigo-950 to-slate-950 flex items-center justify-center">
          <div className="w-full h-full opacity-30 bg-[radial-gradient(#38bdf8_1px,transparent_1px)] [background-size:16px_16px]"></div>
        </div>

        {/* Top bar info */}
        <div className="relative z-10 flex items-center justify-between text-[11px] text-slate-300">
          <span className="bg-black/60 px-2 py-0.5 rounded backdrop-blur font-mono">
            1080x1920 @ 30fps
          </span>
          <span className="bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded backdrop-blur">
            Frame-Accurate
          </span>
        </div>

        {/* Content Card (Center) */}
        <div className="relative z-10 bg-slate-900/90 border border-slate-700/80 rounded-2xl p-4 shadow-xl backdrop-blur-md space-y-2">
          <div className="flex items-center gap-2">
            <div className="w-7 h-7 rounded-full bg-emerald-500 text-white flex items-center justify-center font-bold text-xs">
              C
            </div>
            <div>
              <div className="font-bold text-xs text-slate-100">@creator_vn</div>
              <div className="text-[10px] text-slate-400">Threads Author</div>
            </div>
          </div>
          <p className="text-xs text-slate-200 leading-relaxed">
            Dựng video ngắn từ Threads cực kỳ nhanh với Antares ThreadClip Studio! Đầy đủ voice và phụ đề chuẩn xác.
          </p>
        </div>

        {/* Captions Overlay (Bottom) */}
        <div className="relative z-10 text-center pb-6">
          <span className="bg-black/80 text-amber-300 font-bold text-sm px-3 py-1 rounded shadow-lg border border-amber-400/20">
            Dựng video ngắn cực kỳ nhanh!
          </span>
        </div>
      </div>

      {/* Control Panel */}
      <div className="w-80 bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-5">
        <div>
          <h3 className="text-lg font-bold text-slate-100">Xem trước Video (S06)</h3>
          <p className="text-xs text-slate-400 mt-1">
            Mô phỏng chính xác khung hình đầu ra tỉ lệ 9:16 Shorts/Reels/TikTok.
          </p>
        </div>

        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs font-mono text-slate-300">
            <span>Thời gian:</span>
            <span className="text-emerald-400 font-semibold">00:00:01:15</span>
          </div>
          <input
            type="range"
            min="0"
            max="100"
            value={progress}
            onChange={(e) => setProgress(Number(e.target.value))}
            className="w-full accent-emerald-500 bg-slate-800 h-1.5 rounded-lg"
          />
        </div>

        <div className="flex items-center justify-center gap-3 pt-2">
          <button
            onClick={() => setProgress(0)}
            className="p-2.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg transition"
            title="Về đầu"
          >
            <RotateCcw className="w-4 h-4" />
          </button>
          <button
            onClick={() => setIsPlaying(!isPlaying)}
            className="p-3 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition shadow-md"
          >
            {isPlaying ? <Pause className="w-5 h-5" /> : <Play className="w-5 h-5" />}
          </button>
          <button
            className="p-2.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg transition"
            title="Chụp ảnh Thumbnail"
          >
            <Camera className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
};
