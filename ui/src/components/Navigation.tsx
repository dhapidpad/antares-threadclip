import React from 'react';
import {
  FolderKanban,
  Download,
  MessageSquareQuote,
  Image,
  Film,
  PlaySquare,
  Mic,
  Clapperboard,
  History,
  Settings,
  Terminal,
} from 'lucide-react';

export type ScreenId =
  | 'dashboard'
  | 'import'
  | 'content'
  | 'assets'
  | 'editor'
  | 'preview'
  | 'voice'
  | 'render'
  | 'history'
  | 'settings'
  | 'logs';

interface NavigationProps {
  currentScreen: ScreenId;
  onSelectScreen: (screen: ScreenId) => void;
  lang: 'vi' | 'en';
}

export const Navigation: React.FC<NavigationProps> = ({
  currentScreen,
  onSelectScreen,
  lang,
}) => {
  const items = [
    { id: 'dashboard', labelVi: 'Dự án (S01)', labelEn: 'Projects (S01)', icon: FolderKanban },
    { id: 'import', labelVi: 'Nhập Threads (S02)', labelEn: 'Import (S02)', icon: Download },
    { id: 'content', labelVi: 'Thư viện bài (S03)', labelEn: 'Content (S03)', icon: MessageSquareQuote },
    { id: 'assets', labelVi: 'Tài nguyên (S04)', labelEn: 'Assets (S04)', icon: Image },
    { id: 'editor', labelVi: 'Dựng Video (S05)', labelEn: 'Timeline (S05)', icon: Film },
    { id: 'preview', labelVi: 'Xem trước (S06)', labelEn: 'Preview (S06)', icon: PlaySquare },
    { id: 'voice', labelVi: 'Giọng & Phụ đề (S07)', labelEn: 'Voice & SRT (S07)', icon: Mic },
    { id: 'render', labelVi: 'Hàng đợi xuất (S08)', labelEn: 'Render Queue (S08)', icon: Clapperboard },
    { id: 'history', labelVi: 'Lịch sử (S09)', labelEn: 'History (S09)', icon: History },
    { id: 'settings', labelVi: 'Cài đặt (S10)', labelEn: 'Settings (S10)', icon: Settings },
    { id: 'logs', labelVi: 'Nhật ký (S11)', labelEn: 'Logs (S11)', icon: Terminal },
  ] as const;

  return (
    <nav className="w-64 bg-slate-900 border-r border-slate-800 flex flex-col flex-shrink-0">
      <div className="p-4 border-b border-slate-800 flex items-center gap-2">
        <div className="w-8 h-8 rounded-lg bg-emerald-500/20 text-emerald-400 flex items-center justify-center font-bold">
          A
        </div>
        <div>
          <h1 className="font-semibold text-sm text-slate-100">Antares Studio</h1>
          <p className="text-xs text-slate-400">ThreadClip v3.0.0</p>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto py-2 px-2 space-y-1">
        {items.map((item) => {
          const Icon = item.icon;
          const isActive = currentScreen === item.id;
          const label = lang === 'vi' ? item.labelVi : item.labelEn;

          return (
            <button
              key={item.id}
              onClick={() => onSelectScreen(item.id)}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-md text-xs font-medium transition-colors ${
                isActive
                  ? 'bg-emerald-600 text-white shadow-sm'
                  : 'text-slate-300 hover:bg-slate-800 hover:text-slate-100'
              }`}
            >
              <Icon className="w-4 h-4 flex-shrink-0" />
              <span className="truncate">{label}</span>
            </button>
          );
        })}
      </div>

      <div className="p-3 border-t border-slate-800 bg-slate-900/50">
        <div className="text-[10px] text-slate-400 text-center">
          Local-first • SQLite FTS5 • FFmpeg
        </div>
      </div>
    </nav>
  );
};
