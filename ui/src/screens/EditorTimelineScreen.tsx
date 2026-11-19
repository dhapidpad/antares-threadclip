import React, { useState } from 'react';
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  Undo2,
  Redo2,
  Scissors,
  Trash2,
  ZoomIn,
  ZoomOut,
  Magnet,
  Volume2,
} from 'lucide-react';

export const EditorTimelineScreen: React.FC = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentFrame, setCurrentFrame] = useState(45);
  const [zoom, setZoom] = useState(1);
  const [snapEnabled, setSnapEnabled] = useState(true);

  const tracks = [
    {
      id: 't-video',
      name: 'Track Nền (Video)',
      clips: [
        { id: 'c1', title: 'gameplay_loop.mp4', start: 0, duration: 300, color: 'bg-indigo-600/80 border-indigo-400' },
      ],
    },
    {
      id: 't-text',
      name: 'Track Thẻ chữ (Cards)',
      clips: [
        { id: 'c2', title: 'Card #1: Bài viết gốc', start: 0, duration: 90, color: 'bg-emerald-600/80 border-emerald-400' },
        { id: 'c3', title: 'Card #2: Phản hồi tác giả', start: 90, duration: 120, color: 'bg-emerald-700/80 border-emerald-400' },
      ],
    },
    {
      id: 't-voice',
      name: 'Track Giọng đọc (TTS)',
      clips: [
        { id: 'c4', title: 'TTS: Giọng nữ Bắc (vi-VN)', start: 0, duration: 88, color: 'bg-purple-600/80 border-purple-400' },
        { id: 'c5', title: 'TTS: Giọng nam Nam (vi-VN)', start: 90, duration: 115, color: 'bg-purple-600/80 border-purple-400' },
      ],
    },
    {
      id: 't-music',
      name: 'Track Nhạc nền (Ducking)',
      clips: [
        { id: 'c6', title: 'lofi_chill.mp3 (Ducking -12dB)', start: 0, duration: 300, color: 'bg-amber-600/80 border-amber-400' },
      ],
    },
  ];

  const totalFrames = 300;
  const pixelsPerFrame = 2.5 * zoom;

  return (
    <div className="flex flex-col h-full bg-slate-950 overflow-hidden">
      {/* Top Toolbar */}
      <div className="h-12 border-b border-slate-800 bg-slate-900 px-4 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <button
            onClick={() => setIsPlaying(!isPlaying)}
            className="p-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition"
            title="Space"
          >
            {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
          </button>
          <button
            onClick={() => setCurrentFrame(0)}
            className="p-2 hover:bg-slate-800 text-slate-300 rounded-lg transition"
            title="Về đầu"
          >
            <SkipBack className="w-4 h-4" />
          </button>
          <button
            onClick={() => setCurrentFrame(totalFrames)}
            className="p-2 hover:bg-slate-800 text-slate-300 rounded-lg transition"
            title="Về cuối"
          >
            <SkipForward className="w-4 h-4" />
          </button>

          <div className="h-4 w-px bg-slate-800 mx-1"></div>

          <button className="p-2 hover:bg-slate-800 text-slate-300 rounded-lg transition" title="Undo (Ctrl+Z)">
            <Undo2 className="w-4 h-4" />
          </button>
          <button className="p-2 hover:bg-slate-800 text-slate-300 rounded-lg transition" title="Redo (Ctrl+Y)">
            <Redo2 className="w-4 h-4" />
          </button>
          <button className="p-2 hover:bg-slate-800 text-slate-300 rounded-lg transition" title="Tách clip (S)">
            <Scissors className="w-4 h-4" />
          </button>
          <button className="p-2 hover:bg-slate-800 text-rose-400 rounded-lg transition" title="Xóa (Del)">
            <Trash2 className="w-4 h-4" />
          </button>
        </div>

        {/* Playhead Timecode Display */}
        <div className="flex items-center gap-4">
          <div className="bg-slate-950 border border-slate-800 px-3 py-1 rounded font-mono text-xs text-emerald-400">
            Khung: <strong className="text-slate-100">{currentFrame}</strong> / {totalFrames} (00:00:01:15 @ 30fps)
          </div>

          <div className="flex items-center gap-1">
            <button
              onClick={() => setSnapEnabled(!snapEnabled)}
              className={`p-1.5 rounded transition ${snapEnabled ? 'bg-emerald-600/20 text-emerald-400' : 'text-slate-500'}`}
              title="Snap vào frame"
            >
              <Magnet className="w-4 h-4" />
            </button>
            <button onClick={() => setZoom((z) => Math.max(0.5, z - 0.2))} className="p-1.5 hover:bg-slate-800 text-slate-400 rounded">
              <ZoomOut className="w-4 h-4" />
            </button>
            <button onClick={() => setZoom((z) => Math.min(3, z + 0.2))} className="p-1.5 hover:bg-slate-800 text-slate-400 rounded">
              <ZoomIn className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Main Timeline Workspace */}
      <div className="flex-1 flex overflow-hidden">
        {/* Track Headers */}
        <div className="w-56 bg-slate-900 border-r border-slate-800 flex flex-col flex-shrink-0 z-10">
          <div className="h-8 border-b border-slate-800 bg-slate-900/80 px-3 flex items-center text-[11px] font-semibold text-slate-400">
            Các lớp (Tracks)
          </div>
          {tracks.map((t) => (
            <div
              key={t.id}
              className="h-16 border-b border-slate-800/80 px-3 flex items-center justify-between text-xs text-slate-300 bg-slate-900/40"
            >
              <span className="truncate">{t.name}</span>
              <Volume2 className="w-3.5 h-3.5 text-slate-500" />
            </div>
          ))}
        </div>

        {/* Timeline Tracks & Ruler */}
        <div className="flex-1 overflow-x-auto overflow-y-hidden relative bg-slate-950">
          {/* Ruler */}
          <div className="h-8 border-b border-slate-800 bg-slate-900/60 relative flex items-end select-none">
            {Array.from({ length: Math.ceil(totalFrames / 30) + 1 }).map((_, sec) => {
              const frame = sec * 30;
              const left = frame * pixelsPerFrame;
              return (
                <div
                  key={sec}
                  style={{ left: `${left}px` }}
                  className="absolute bottom-0 text-[10px] font-mono text-slate-400 border-l border-slate-700 pl-1 h-4 flex items-center"
                >
                  {sec}s ({frame}f)
                </div>
              );
            })}
          </div>

          {/* Tracks Area */}
          <div className="relative">
            {tracks.map((t) => (
              <div key={t.id} className="h-16 border-b border-slate-800/60 relative">
                {t.clips.map((clip) => {
                  const left = clip.start * pixelsPerFrame;
                  const width = clip.duration * pixelsPerFrame;

                  return (
                    <div
                      key={clip.id}
                      style={{ left: `${left}px`, width: `${width}px` }}
                      className={`absolute top-2 bottom-2 rounded-md border text-[11px] font-medium text-white px-2 py-1 flex items-center justify-between truncate cursor-pointer shadow-sm select-none ${clip.color}`}
                    >
                      <span className="truncate">{clip.title}</span>
                      <span className="text-[9px] opacity-70 font-mono ml-1">{clip.duration}f</span>
                    </div>
                  );
                })}
              </div>
            ))}

            {/* Playhead Indicator */}
            <div
              style={{ left: `${currentFrame * pixelsPerFrame}px` }}
              className="absolute top-0 bottom-0 w-0.5 bg-rose-500 pointer-events-none z-20 shadow-[0_0_8px_rgba(244,63,94,0.8)]"
            >
              <div className="w-3 h-3 bg-rose-500 -ml-[5px] -mt-1.5 rotate-45 rounded-[1px]"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
