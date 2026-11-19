import React, { useState } from 'react';
import { UploadCloud, CheckCircle2, AlertTriangle, FileJson, Gauge, RefreshCw } from 'lucide-react';

interface ImportScreenProps {
  onImportComplete?: () => void;
}

export const ImportScreen: React.FC<ImportScreenProps> = () => {
  const [jsonInput, setJsonInput] = useState('');
  const [importing, setImporting] = useState(false);
  const [result, setResult] = useState<{ posts: number; replies: number } | null>({
    posts: 20,
    replies: 420,
  });

  const handleImport = () => {
    setImporting(true);
    setTimeout(() => {
      setImporting(false);
      setResult({ posts: 20, replies: 420 });
    }, 600);
  };

  return (
    <div className="p-8 max-w-5xl mx-auto space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight text-slate-100">
          Tài khoản & Nhập Threads (S02)
        </h2>
        <p className="text-sm text-slate-400">
          Kết nối tài khoản Threads qua OAuth hoặc nhập dữ liệu ngoại tuyến từ file JSON mẫu.
        </p>
      </div>

      {/* UX Warning (Spec 5.5) */}
      <div className="bg-amber-500/10 border border-amber-500/30 rounded-lg p-4 flex items-start gap-3 text-amber-200 text-xs">
        <AlertTriangle className="w-5 h-5 flex-shrink-0 text-amber-400 mt-0.5" />
        <div>
          <strong className="font-semibold">Lưu ý bản quyền & sửa bài:</strong> Threads không hỗ
          trợ sửa bài đã đăng qua API. Bản video sử dụng nội dung chính xác tại thời điểm import.
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* OAuth Section */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold text-slate-100 flex items-center gap-2">
              <span className="w-2.5 h-2.5 rounded-full bg-emerald-500"></span>
              Threads OAuth Desktop Loopback
            </h3>
            <span className="text-[11px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded-full font-medium">
              Đã kết nối
            </span>
          </div>

          <div className="text-xs text-slate-400 space-y-2">
            <p>Tài khoản: <strong className="text-slate-200">@creator_vn</strong></p>
            <p>Token ID: <span className="font-mono text-slate-400">keychain://threads/user_token</span></p>
            <p>Hạn dùng còn lại: <span className="text-emerald-400 font-medium">58 ngày</span> (Tự động gia hạn trước 14 ngày)</p>
          </div>

          {/* Rate Limiter Token Bucket */}
          <div className="bg-slate-950/60 border border-slate-800/80 rounded-lg p-3 space-y-2">
            <div className="flex items-center justify-between text-xs">
              <span className="flex items-center gap-1.5 text-slate-300">
                <Gauge className="w-4 h-4 text-blue-400" />
                Hạn mức Token Bucket
              </span>
              <span className="font-mono text-emerald-400">19 / 20 requests</span>
            </div>
            <div className="w-full bg-slate-800 h-1.5 rounded-full overflow-hidden">
              <div className="bg-emerald-500 h-full w-[95%]"></div>
            </div>
            <p className="text-[10px] text-slate-400">
              Tốc độ nạp lại: 0.055 req/s (~200 req/giờ). Đảm bảo không bị Meta rate-limit.
            </p>
          </div>

          <button className="w-full py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium rounded-lg transition flex items-center justify-center gap-2">
            <RefreshCw className="w-3.5 h-3.5" />
            Đồng bộ bài viết mới nhất
          </button>
        </div>

        {/* Offline Fallback Section */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold text-slate-100 flex items-center gap-2">
              <FileJson className="w-4 h-4 text-emerald-400" />
              Nhập thủ công Offline (Fallback V-06)
            </h3>
            <span className="text-[11px] text-slate-400">JSON Fixture</span>
          </div>

          <p className="text-xs text-slate-400">
            Dán nội dung JSON hoặc tải file mẫu từ <code className="text-emerald-400">testdata/threads_sample.json</code> (hỗ trợ phân cấp lồng sâu &gt;= 4 cấp).
          </p>

          <textarea
            value={jsonInput}
            onChange={(e) => setJsonInput(e.target.value)}
            placeholder="Dán JSON threads_sample tại đây hoặc để trống để dùng file mẫu..."
            rows={4}
            className="w-full bg-slate-950 border border-slate-800 rounded-lg p-3 text-xs text-slate-200 font-mono focus:outline-none focus:border-emerald-500 resize-none"
          />

          <button
            onClick={handleImport}
            disabled={importing}
            className="w-full py-2.5 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white text-xs font-medium rounded-lg transition flex items-center justify-center gap-2 shadow-sm"
          >
            <UploadCloud className="w-4 h-4" />
            {importing ? 'Đang phân tích cây thảo luận...' : 'Bắt đầu nạp vào SQLite'}
          </button>

          {result && (
            <div className="bg-emerald-500/10 border border-emerald-500/20 rounded-lg p-3 flex items-center gap-3 text-xs text-emerald-300">
              <CheckCircle2 className="w-4 h-4 flex-shrink-0 text-emerald-400" />
              <div>
                Đã nạp thành công <strong>{result.posts} bài viết</strong> và{' '}
                <strong>{result.replies} bình luận</strong> vào cơ sở dữ liệu!
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
