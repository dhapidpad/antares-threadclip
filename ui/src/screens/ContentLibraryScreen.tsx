import React, { useState } from 'react';
import {
  Search,
  Filter,
  PlusCircle,
  Tag,
  CornerDownRight,
  Heart,
  MessageCircle,
  HelpCircle,
} from 'lucide-react';

interface ContentItem {
  id: string;
  kind: 'post' | 'reply';
  author: string;
  text: string;
  depth: number;
  replies: number;
  likes: number;
  isAuthorReply: boolean;
  scoreKind: 'real' | 'heuristic';
  score: number;
}

export const ContentLibraryScreen: React.FC = () => {
  const [search, setSearch] = useState('');
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [showExplanation, setShowExplanation] = useState<string | null>(null);

  const sampleItems: ContentItem[] = [
    {
      id: 'Df5D-Hak-d3',
      kind: 'post',
      author: 'cung_khong_iu_toan_lam',
      text: 'như này có nên chan không mng ??',
      depth: 0,
      replies: 34,
      likes: 602,
      isAuthorReply: true,
      scoreKind: 'real',
      score: 42.6,
    },
    {
      id: 'Df6CfHQH8Bw',
      kind: 'reply',
      author: 'trbmih_',
      text: 'chan luôn đi',
      depth: 1,
      replies: 2,
      likes: 1,
      isAuthorReply: false,
      scoreKind: 'real',
      score: 2.3,
    },
    {
      id: 'Df6H_yrDyCp',
      kind: 'reply',
      author: 'cung_khong_iu_toan_lam',
      text: 'chan xong mat xac 😭',
      depth: 2,
      replies: 1,
      likes: 2,
      isAuthorReply: true,
      scoreKind: 'real',
      score: 5.8,
    },
    {
      id: 'Df6M7HnkYhU',
      kind: 'reply',
      author: 'trbmih_',
      text: 'bạn sợ à',
      depth: 3,
      replies: 1,
      likes: 1,
      isAuthorReply: false,
      scoreKind: 'heuristic',
      score: 1.9,
    },
    {
      id: 'Df6Q50ik5ur',
      kind: 'reply',
      author: 'cung_khong_iu_toan_lam',
      text: 'sợ ngta dỗi b ạ 😔🥀',
      depth: 4,
      replies: 0,
      likes: 1,
      isAuthorReply: true,
      scoreKind: 'heuristic',
      score: 4.5,
    },
    {
      id: 'Df55cnuEg4U',
      kind: 'reply',
      author: 'anhk.lyz_ddabi',
      text: 'chan đi bà oii',
      depth: 1,
      replies: 1,
      likes: 1,
      isAuthorReply: false,
      scoreKind: 'real',
      score: 2.1,
    },
    {
      id: 'Df59QL_mYYq',
      kind: 'reply',
      author: 'cung_khong_iu_toan_lam',
      text: 'kèo dưới nên chan hơi khê 😔🥀',
      depth: 2,
      replies: 0,
      likes: 1,
      isAuthorReply: true,
      scoreKind: 'real',
      score: 5.2,
    },
    {
      id: 'post_1',
      kind: 'post',
      author: 'creator_vn',
      text: 'Hướng dẫn dựng video ngắn từ Threads với Antares Studio local-first! Tối ưu frame-accurate và ducking âm thanh.',
      depth: 0,
      replies: 21,
      likes: 125,
      isAuthorReply: true,
      scoreKind: 'real',
      score: 18.4,
    },
    {
      id: 'r_1_1',
      kind: 'reply',
      author: 'user_1',
      text: 'Bình luận cấp 1: Tuyệt vời quá, có thể xuất video 9:16 trực tiếp cho Shorts/Reels không?',
      depth: 1,
      replies: 2,
      likes: 15,
      isAuthorReply: false,
      scoreKind: 'real',
      score: 5.2,
    },
    {
      id: 'r_1_1_1',
      kind: 'reply',
      author: 'creator_vn',
      text: 'Phản hồi từ tác giả: Hoàn toàn được bạn nhé, preset mặc định là 1080x1920 30fps chuẩn Shorts!',
      depth: 2,
      replies: 1,
      likes: 22,
      isAuthorReply: true,
      scoreKind: 'real',
      score: 9.1,
    },
    {
      id: 'r_1_1_1_1',
      kind: 'reply',
      author: 'viewer_deep_1',
      text: 'Âm thanh tự động giảm khi có giọng đọc đúng không tác giả?',
      depth: 3,
      replies: 1,
      likes: 4,
      isAuthorReply: false,
      scoreKind: 'heuristic',
      score: 0.75,
    },
    {
      id: 'r_1_1_1_1_1',
      kind: 'reply',
      author: 'creator_vn',
      text: 'Đúng rồi, FFmpeg sidechaincompress giảm nhạc nền 12dB khi có lời thoại.',
      depth: 4,
      replies: 0,
      likes: 8,
      isAuthorReply: true,
      scoreKind: 'heuristic',
      score: 0.88,
    },
  ];

  const toggleSelect = (id: string) => {
    const next = new Set(selectedIds);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    setSelectedIds(next);
  };

  const selectAll = () => {
    if (selectedIds.size === sampleItems.length) {
      setSelectedIds(new Set());
    } else {
      setSelectedIds(new Set(sampleItems.map((i) => i.id)));
    }
  };

  return (
    <div className="flex h-full overflow-hidden">
      {/* Left Area: Content List / Thread Tree */}
      <div className="flex-1 flex flex-col border-r border-slate-800 bg-slate-950">
        {/* Header / Action Bar */}
        <div className="p-4 border-b border-slate-800 flex items-center justify-between gap-4 bg-slate-900/40">
          <div className="flex items-center gap-3">
            <input
              type="checkbox"
              checked={selectedIds.size === sampleItems.length && sampleItems.length > 0}
              onChange={selectAll}
              className="rounded bg-slate-800 border-slate-700 text-emerald-500 focus:ring-0"
            />
            <span className="text-xs text-slate-300">
              Đã chọn: <strong className="text-emerald-400">{selectedIds.size}</strong> / {sampleItems.length}
            </span>
          </div>

          <div className="flex items-center gap-2">
            <button
              disabled={selectedIds.size === 0}
              className="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-40 text-white rounded-lg text-xs font-medium transition"
            >
              <PlusCircle className="w-3.5 h-3.5" />
              Thêm vào Dự án
            </button>
            <button
              disabled={selectedIds.size === 0}
              className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 disabled:opacity-40 text-slate-300 rounded-lg text-xs font-medium transition"
            >
              <Tag className="w-3.5 h-3.5" />
              Gắn nhãn
            </button>
          </div>
        </div>

        {/* List of items */}
        <div className="flex-1 overflow-y-auto p-4 space-y-2">
          {sampleItems.map((item) => {
            const isSelected = selectedIds.has(item.id);
            const indentPx = item.depth * 24;

            return (
              <div
                key={item.id}
                style={{ marginLeft: `${indentPx}px` }}
                className={`p-3 rounded-lg border transition flex items-start gap-3 ${
                  isSelected
                    ? 'bg-slate-900 border-emerald-500/60'
                    : 'bg-slate-900/70 border-slate-800/80 hover:border-slate-700'
                }`}
              >
                <input
                  type="checkbox"
                  checked={isSelected}
                  onChange={() => toggleSelect(item.id)}
                  className="mt-1 rounded bg-slate-800 border-slate-700 text-emerald-500 focus:ring-0"
                />

                <div className="flex-1 space-y-1.5">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      {item.depth > 0 && (
                        <CornerDownRight className="w-3.5 h-3.5 text-slate-500" />
                      )}
                      <span className="font-semibold text-xs text-slate-200">
                        @{item.author}
                      </span>
                      {item.isAuthorReply && (
                        <span className="text-[10px] bg-blue-500/10 text-blue-400 border border-blue-500/20 px-1.5 py-0.2 rounded font-medium">
                          Tác giả
                        </span>
                      )}
                      <span className="text-[10px] text-slate-400">
                        (Cấp {item.depth})
                      </span>
                    </div>

                    {/* Ranking Badge */}
                    <div className="flex items-center gap-1.5">
                      {item.scoreKind === 'real' ? (
                        <span className="text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded-full flex items-center gap-1">
                          ● Dữ liệu thật ({item.score.toFixed(1)})
                        </span>
                      ) : (
                        <span className="text-[10px] bg-amber-500/10 text-amber-400 border border-amber-500/20 px-2 py-0.5 rounded-full flex items-center gap-1">
                          ◌ Ước lượng ({item.score.toFixed(2)})
                        </span>
                      )}
                      <button
                        onClick={() =>
                          setShowExplanation(
                            showExplanation === item.id ? null : item.id
                          )
                        }
                        className="text-slate-500 hover:text-slate-300"
                        title="Vì sao có điểm này?"
                      >
                        <HelpCircle className="w-3.5 h-3.5" />
                      </button>
                    </div>
                  </div>

                  <p className="text-xs text-slate-300 leading-relaxed">{item.text}</p>

                  <div className="flex items-center gap-4 text-[11px] text-slate-400 pt-1">
                    <span className="flex items-center gap-1">
                      <Heart className="w-3 h-3 text-rose-400" />
                      {item.likes}
                    </span>
                    <span className="flex items-center gap-1">
                      <MessageCircle className="w-3 h-3 text-blue-400" />
                      {item.replies}
                    </span>
                  </div>

                  {showExplanation === item.id && (
                    <div className="mt-2 p-2.5 bg-slate-950 border border-slate-800 rounded text-[11px] text-slate-300 space-y-1">
                      <div className="font-semibold text-slate-200">
                        Chi tiết chấm điểm:
                      </div>
                      <div>
                        • Loại điểm: {item.scoreKind === 'real' ? 'Real-score (từ chỉ số engagement thật)' : 'Heuristic-score (dựa trên độ dài, từ khóa, độ sâu)'}
                      </div>
                      <div>• Điểm tổng: {item.score.toFixed(2)}</div>
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Right Area: Search & Filter Panel */}
      <div className="w-80 bg-slate-900 border-l border-slate-800 p-5 space-y-6 overflow-y-auto">
        <div className="flex items-center gap-2 text-sm font-semibold text-slate-100">
          <Filter className="w-4 h-4 text-emerald-400" />
          Bộ lọc tìm kiếm (FTS5)
        </div>

        <div className="space-y-4 text-xs">
          <div>
            <label className="block text-slate-400 mb-1">Từ khóa (FTS5 có/không dấu)</label>
            <div className="relative">
              <Search className="w-3.5 h-3.5 absolute left-3 top-2.5 text-slate-500" />
              <input
                type="text"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Tìm kiếm nội dung..."
                className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-8 pr-3 py-1.5 text-xs text-slate-100 focus:outline-none focus:border-emerald-500"
              />
            </div>
          </div>

          <div>
            <label className="block text-slate-400 mb-1">Xếp hạng theo</label>
            <select className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-100 focus:outline-none focus:border-emerald-500">
              <option>Real-score cao nhất</option>
              <option>Heuristic-score cao nhất</option>
              <option>Thời gian mới nhất</option>
              <option>Nhiều phản hồi nhất</option>
            </select>
          </div>

          <div>
            <label className="block text-slate-400 mb-1">Độ sâu bình luận (Depth)</label>
            <div className="grid grid-cols-2 gap-2">
              <input
                type="number"
                placeholder="Tối thiểu: 0"
                className="bg-slate-950 border border-slate-800 rounded-lg px-2 py-1 text-xs text-slate-100"
              />
              <input
                type="number"
                placeholder="Tối đa: 4"
                className="bg-slate-950 border border-slate-800 rounded-lg px-2 py-1 text-xs text-slate-100"
              />
            </div>
          </div>

          <div className="pt-2 border-t border-slate-800">
            <label className="flex items-center gap-2 text-slate-300">
              <input
                type="checkbox"
                defaultChecked
                className="rounded bg-slate-800 border-slate-700 text-emerald-500"
              />
              Ưu tiên phản hồi của tác giả
            </label>
          </div>
        </div>
      </div>
    </div>
  );
};
