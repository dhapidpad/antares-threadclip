import React, { useState } from 'react';
import { Plus, Folder, Calendar, Copy, Trash2, ArrowRight } from 'lucide-react';

interface DashboardScreenProps {
  onOpenProject: (name: string) => void;
}

export const DashboardScreen: React.FC<DashboardScreenProps> = ({ onOpenProject }) => {
  const [projects, setProjects] = useState([
    {
      id: 'proj_1',
      name: 'AI & Sáng Tạo Video Shorts #1',
      updatedAt: 'Hôm nay, 10:30',
      duration: '45s',
      clips: 6,
    },
    {
      id: 'proj_2',
      name: 'Top Phản Hồi Hay Tuần Qua',
      updatedAt: 'Hôm qua, 18:20',
      duration: '30s',
      clips: 4,
    },
  ]);

  const [newProjectName, setNewProjectName] = useState('');
  const [isModalOpen, setIsModalOpen] = useState(false);

  const handleCreate = () => {
    if (!newProjectName.trim()) return;
    const newP = {
      id: `proj_${Date.now()}`,
      name: newProjectName.trim(),
      updatedAt: 'Vừa xong',
      duration: '0s',
      clips: 0,
    };
    setProjects([newP, ...projects]);
    setNewProjectName('');
    setIsModalOpen(false);
    onOpenProject(newP.name);
  };

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight text-slate-100">Dự án của bạn (S01)</h2>
          <p className="text-sm text-slate-400">
            Quản lý các dự án video ngắn từ bài viết Threads. Định dạng project.json tự đủ.
          </p>
        </div>
        <button
          onClick={() => setIsModalOpen(true)}
          className="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-white px-4 py-2 rounded-lg text-sm font-medium shadow-sm transition"
        >
          <Plus className="w-4 h-4" />
          Tạo dự án mới
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {projects.map((p) => (
          <div
            key={p.id}
            className="bg-slate-900 border border-slate-800 hover:border-slate-700 rounded-xl p-5 flex flex-col justify-between group transition"
          >
            <div className="space-y-3">
              <div className="w-10 h-10 rounded-lg bg-blue-500/10 text-blue-400 flex items-center justify-center">
                <Folder className="w-5 h-5" />
              </div>
              <div>
                <h3 className="font-semibold text-slate-100 group-hover:text-emerald-400 transition">
                  {p.name}
                </h3>
                <div className="flex items-center gap-3 text-xs text-slate-400 mt-1">
                  <span className="flex items-center gap-1">
                    <Calendar className="w-3 h-3" />
                    {p.updatedAt}
                  </span>
                  <span>• {p.duration}</span>
                  <span>• {p.clips} clips</span>
                </div>
              </div>
            </div>

            <div className="flex items-center justify-between pt-4 mt-4 border-t border-slate-800/80">
              <div className="flex items-center gap-1">
                <button
                  title="Nhân bản"
                  className="p-1.5 hover:bg-slate-800 text-slate-400 hover:text-slate-200 rounded"
                >
                  <Copy className="w-3.5 h-3.5" />
                </button>
                <button
                  title="Xóa"
                  className="p-1.5 hover:bg-slate-800 text-slate-400 hover:text-rose-400 rounded"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                </button>
              </div>
              <button
                onClick={() => onOpenProject(p.name)}
                className="flex items-center gap-1.5 text-xs font-medium text-emerald-400 hover:text-emerald-300"
              >
                Mở biên tập
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        ))}
      </div>

      {isModalOpen && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
          <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 w-full max-w-md space-y-4 shadow-xl">
            <h3 className="text-lg font-bold text-slate-100">Tạo dự án mới</h3>
            <div>
              <label className="block text-xs font-medium text-slate-300 mb-1">
                Tên dự án
              </label>
              <input
                type="text"
                autoFocus
                value={newProjectName}
                onChange={(e) => setNewProjectName(e.target.value)}
                placeholder="VD: Shorts Review Threads #1"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-sm text-slate-100 focus:outline-none focus:border-emerald-500"
                onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
              />
            </div>
            <div className="flex justify-end gap-2 pt-2">
              <button
                onClick={() => setIsModalOpen(false)}
                className="px-4 py-2 text-xs font-medium text-slate-400 hover:text-slate-200"
              >
                Hủy
              </button>
              <button
                onClick={handleCreate}
                className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-medium"
              >
                Tạo và mở
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
