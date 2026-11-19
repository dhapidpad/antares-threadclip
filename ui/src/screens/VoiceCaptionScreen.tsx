import React, { useState } from 'react';
import { Volume2, Play, Download, AlertCircle, Sparkles } from 'lucide-react';

export const VoiceCaptionScreen: React.FC = () => {
  const [selectedVoice, setSelectedVoice] = useState('vi-VN-Neural2-A');
  const [rate, setRate] = useState(1.0);
  const [pitch, setPitch] = useState(0.0);
  const [text, setText] = useState(
    'Dựng video ngắn từ Threads cực kỳ nhanh với Antares ThreadClip Studio! Đầy đủ voice và phụ đề chuẩn xác.'
  );

  const cues = [
    { start: '00:00:00,000', end: '00:00:02,100', text: 'Dựng video ngắn từ Threads', estimated: false },
    { start: '00:00:02,100', end: '00:00:04,500', text: 'cực kỳ nhanh với Antares Studio!', estimated: false },
    { start: '00:00:04,500', end: '00:00:06,800', text: 'Đầy đủ voice và phụ đề chuẩn xác.', estimated: true },
  ];

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight text-slate-100">
          Giọng đọc & Phụ đề Caption (S07)
        </h2>
        <p className="text-sm text-slate-400">
          Thiết lập TTS tiếng Việt đa giọng, tạo phụ đề SRT/WebVTT tự động và đồng bộ thời gian.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* TTS Config */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <h3 className="font-semibold text-slate-100 flex items-center gap-2 text-sm">
            <Volume2 className="w-4 h-4 text-emerald-400" />
            Cấu hình Giọng đọc (TTS Provider)
          </h3>

          <div>
            <label className="block text-xs font-medium text-slate-300 mb-1">
              Giọng tiếng Việt (Speaker)
            </label>
            <select
              value={selectedVoice}
              onChange={(e) => setSelectedVoice(e.target.value)}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-xs text-slate-100 focus:outline-none focus:border-emerald-500"
            >
              <option value="vi-VN-Neural2-A">Giọng nữ miền Bắc (vi-VN-Neural2-A)</option>
              <option value="vi-VN-Neural2-D">Giọng nam miền Nam (vi-VN-Neural2-D)</option>
              <option value="vi-VN-Wavenet-A">Giọng nữ miền Nam (vi-VN-Wavenet-A)</option>
              <option value="mock-sine">Mock Voice (Offline Test Sine Wave)</option>
            </select>
          </div>

          <div className="space-y-3">
            <div>
              <div className="flex justify-between text-xs text-slate-300 mb-1">
                <span>Tốc độ đọc (Speed)</span>
                <span className="font-mono text-emerald-400">{rate.toFixed(1)}x</span>
              </div>
              <input
                type="range"
                min="0.5"
                max="1.5"
                step="0.1"
                value={rate}
                onChange={(e) => setRate(parseFloat(e.target.value))}
                className="w-full accent-emerald-500 bg-slate-800 h-1.5 rounded-lg"
              />
            </div>

            <div>
              <div className="flex justify-between text-xs text-slate-300 mb-1">
                <span>Cao độ (Pitch)</span>
                <span className="font-mono text-emerald-400">{pitch.toFixed(1)}</span>
              </div>
              <input
                type="range"
                min="-5"
                max="5"
                step="0.5"
                value={pitch}
                onChange={(e) => setPitch(parseFloat(e.target.value))}
                className="w-full accent-emerald-500 bg-slate-800 h-1.5 rounded-lg"
              />
            </div>
          </div>

          <div>
            <label className="block text-xs font-medium text-slate-300 mb-1">
              Văn bản cần đọc
            </label>
            <textarea
              value={text}
              onChange={(e) => setText(e.target.value)}
              rows={3}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg p-3 text-xs text-slate-200 focus:outline-none focus:border-emerald-500 resize-none"
            />
          </div>

          <button className="w-full py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-medium flex items-center justify-center gap-2 transition shadow-sm">
            <Play className="w-3.5 h-3.5" />
            Nghe thử giọng đọc
          </button>
        </div>

        {/* Captions Table */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4 flex flex-col justify-between">
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <h3 className="font-semibold text-slate-100 flex items-center gap-2 text-sm">
                <Sparkles className="w-4 h-4 text-amber-400" />
                Bảng Phụ đề Đồng bộ
              </h3>
              <div className="flex items-center gap-1.5">
                <button className="text-[11px] bg-slate-800 hover:bg-slate-700 text-slate-300 px-2.5 py-1 rounded flex items-center gap-1">
                  <Download className="w-3 h-3" />
                  SRT
                </button>
                <button className="text-[11px] bg-slate-800 hover:bg-slate-700 text-slate-300 px-2.5 py-1 rounded flex items-center gap-1">
                  <Download className="w-3 h-3" />
                  VTT
                </button>
              </div>
            </div>

            <div className="space-y-2">
              {cues.map((cue, idx) => (
                <div
                  key={idx}
                  className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-xs space-y-1"
                >
                  <div className="flex items-center justify-between text-[11px]">
                    <span className="font-mono text-emerald-400">
                      {cue.start} → {cue.end}
                    </span>
                    {cue.estimated && (
                      <span className="text-[10px] bg-amber-500/10 text-amber-400 border border-amber-500/20 px-1.5 py-0.2 rounded font-medium flex items-center gap-1">
                        <AlertCircle className="w-2.5 h-2.5" />
                        timing_estimated
                      </span>
                    )}
                  </div>
                  <div className="text-slate-200">{cue.text}</div>
                </div>
              ))}
            </div>
          </div>

          <div className="text-[11px] text-slate-500 pt-2 border-t border-slate-800">
            Quy tắc chuẩn: Tối đa 2 dòng, &le; 42 ký tự/dòng, hiển thị 1–6s không tràn biên clip.
          </div>
        </div>
      </div>
    </div>
  );
};
