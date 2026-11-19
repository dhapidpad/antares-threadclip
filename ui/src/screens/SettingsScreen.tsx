import React from 'react';
import { Settings, Globe, Database, Sliders, Shield } from 'lucide-react';

interface SettingsScreenProps {
  lang: 'vi' | 'en';
  onToggleLang: () => void;
}

export const SettingsScreen: React.FC<SettingsScreenProps> = ({ lang, onToggleLang }) => {
  return (
    <div className="p-8 max-w-4xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight text-slate-100">
          Cài đặt Ứng dụng (S10)
        </h2>
        <p className="text-sm text-slate-400">
          Tùy chỉnh ngôn ngữ, thư mục lưu trữ cục bộ, công thức xếp hạng và khóa API.
        </p>
      </div>

      <div className="space-y-4">
        {/* Language & Theme */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
          <h3 className="font-semibold text-xs text-slate-100 flex items-center gap-2">
            <Globe className="w-4 h-4 text-emerald-400" />
            Ngôn ngữ & Giao diện (i18n & Theme)
          </h3>
          <div className="flex items-center justify-between text-xs pt-1">
            <span className="text-slate-300">Ngôn ngữ hiển thị</span>
            <button
              onClick={onToggleLang}
              className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-100 rounded-lg font-medium transition"
            >
              {lang === 'vi' ? 'Tiếng Việt (Mặc định)' : 'English'}
            </button>
          </div>
        </div>

        {/* Storage */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
          <h3 className="font-semibold text-xs text-slate-100 flex items-center gap-2">
            <Database className="w-4 h-4 text-blue-400" />
            Lưu trữ Local-first
          </h3>
          <div className="space-y-2 text-xs">
            <label className="block text-slate-400">Thư mục dữ liệu ứng dụng</label>
            <div className="flex gap-2">
              <input
                type="text"
                readOnly
                value="d:/thread/data"
                className="flex-1 bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-300 font-mono"
              />
              <button className="px-3 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg">
                Thay đổi
              </button>
            </div>
          </div>
        </div>

        {/* Ranking Weights */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-4">
          <h3 className="font-semibold text-xs text-slate-100 flex items-center gap-2">
            <Sliders className="w-4 h-4 text-amber-400" />
            Trọng số Chấm điểm Ranking (ranking.toml)
          </h3>
          <div className="grid grid-cols-2 gap-4 text-xs">
            <div>
              <label className="block text-slate-400 mb-1">Trọng số Phản hồi (w_r)</label>
              <input
                type="number"
                defaultValue={1.0}
                step="0.1"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-slate-200"
              />
            </div>
            <div>
              <label className="block text-slate-400 mb-1">Trọng số Lượt thích (w_l)</label>
              <input
                type="number"
                defaultValue={0.5}
                step="0.1"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-slate-200"
              />
            </div>
            <div>
              <label className="block text-slate-400 mb-1">Hệ số suy giảm thời gian (gravity)</label>
              <input
                type="number"
                defaultValue={1.5}
                step="0.1"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-slate-200"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
