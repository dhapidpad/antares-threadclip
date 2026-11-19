import React, { useState } from 'react';
import { Image, Film, Music, Type, ShieldCheck, AlertCircle, Plus, FileText } from 'lucide-react';

interface AssetModel {
  id: string;
  name: string;
  kind: 'image' | 'video' | 'audio' | 'font';
  license: string;
  attribution?: string;
  sha256: string;
}

export const AssetsLibraryScreen: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'memes' | 'backgrounds' | 'audio' | 'fonts'>('memes');

  const assets: AssetModel[] = [
    {
      id: 'a1',
      name: 'reaction_smile.svg',
      kind: 'image',
      license: 'CC0-1.0',
      sha256: 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
    },
    {
      id: 'a2',
      name: 'lofi_chill_beat.mp3',
      kind: 'audio',
      license: 'CC-BY-4.0',
      attribution: 'Music by LofiSleep, FreeMusicArchive',
      sha256: '9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08',
    },
    {
      id: 'a3',
      name: 'minecraft_parkour_bg.mp4',
      kind: 'video',
      license: 'USER-OWNED',
      attribution: 'Recorded by creator_vn',
      sha256: '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
    },
  ];

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight text-slate-100">
            Thư viện Tài nguyên & Giấy phép (S04)
          </h2>
          <p className="text-sm text-slate-400">
            Quản lý backgrounds, memes, âm thanh bản quyền an toàn. Tự động sinh file credits.txt.
          </p>
        </div>

        <button className="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-white px-4 py-2 rounded-lg text-xs font-medium transition shadow-sm">
          <Plus className="w-4 h-4" />
          Thêm tài nguyên mới
        </button>
      </div>

      {/* Tabs */}
      <div className="flex items-center gap-2 border-b border-slate-800">
        <button
          onClick={() => setActiveTab('memes')}
          className={`flex items-center gap-2 px-4 py-2.5 text-xs font-medium border-b-2 transition ${
            activeTab === 'memes'
              ? 'border-emerald-500 text-emerald-400'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Image className="w-4 h-4" />
          Memes & Stickers
        </button>
        <button
          onClick={() => setActiveTab('backgrounds')}
          className={`flex items-center gap-2 px-4 py-2.5 text-xs font-medium border-b-2 transition ${
            activeTab === 'backgrounds'
              ? 'border-emerald-500 text-emerald-400'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Film className="w-4 h-4" />
          Background Videos
        </button>
        <button
          onClick={() => setActiveTab('audio')}
          className={`flex items-center gap-2 px-4 py-2.5 text-xs font-medium border-b-2 transition ${
            activeTab === 'audio'
              ? 'border-emerald-500 text-emerald-400'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Music className="w-4 h-4" />
          Nhạc nền & SFX
        </button>
        <button
          onClick={() => setActiveTab('fonts')}
          className={`flex items-center gap-2 px-4 py-2.5 text-xs font-medium border-b-2 transition ${
            activeTab === 'fonts'
              ? 'border-emerald-500 text-emerald-400'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Type className="w-4 h-4" />
          Fonts tiếng Việt
        </button>
      </div>

      {/* Grid of assets */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {assets.map((asset) => (
          <div
            key={asset.id}
            className="bg-slate-900 border border-slate-800 rounded-xl p-4 space-y-3 flex flex-col justify-between"
          >
            <div className="space-y-2">
              <div className="h-32 bg-slate-950 rounded-lg flex items-center justify-center border border-slate-800/60 overflow-hidden relative group">
                {asset.kind === 'image' && (
                  <span className="text-4xl select-none">😊</span>
                )}
                {asset.kind === 'video' && (
                  <Film className="w-10 h-10 text-slate-600" />
                )}
                {asset.kind === 'audio' && (
                  <Music className="w-10 h-10 text-slate-600" />
                )}
                <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition">
                  <span className="text-xs text-white bg-slate-800/80 px-2.5 py-1 rounded">
                    Xem trước
                  </span>
                </div>
              </div>

              <div>
                <h4 className="font-semibold text-xs text-slate-100 truncate">
                  {asset.name}
                </h4>
                <p className="text-[10px] text-slate-500 font-mono mt-0.5 truncate">
                  SHA256: {asset.sha256}
                </p>
              </div>
            </div>

            <div className="pt-3 border-t border-slate-800/80 space-y-2">
              <div className="flex items-center justify-between text-xs">
                <span className="text-slate-400 text-[11px]">Giấy phép:</span>
                <span className="bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded text-[10px] font-medium flex items-center gap-1">
                  <ShieldCheck className="w-3 h-3" />
                  {asset.license}
                </span>
              </div>

              {asset.attribution && (
                <p className="text-[10px] text-slate-400 italic">
                  Ghi công: {asset.attribution}
                </p>
              )}
            </div>
          </div>
        ))}
      </div>

      <div className="bg-slate-900 border border-slate-800 rounded-xl p-4 flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs text-slate-300">
          <FileText className="w-4 h-4 text-blue-400" />
          <span>Tự động xuất tệp <strong>credits.txt</strong> cho mọi tài nguyên CC-BY khi xuất bản video.</span>
        </div>
        <button className="text-xs text-emerald-400 hover:underline">
          Xem trước credits.txt
        </button>
      </div>
    </div>
  );
};
