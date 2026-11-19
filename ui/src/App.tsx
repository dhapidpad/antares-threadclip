import React, { useState, useEffect } from 'react';
import { Navigation, ScreenId } from './components/Navigation';
import { StatusBar } from './components/StatusBar';
import { DashboardScreen } from './screens/DashboardScreen';
import { ImportScreen } from './screens/ImportScreen';
import { ContentLibraryScreen } from './screens/ContentLibraryScreen';
import { AssetsLibraryScreen } from './screens/AssetsLibraryScreen';
import { EditorTimelineScreen } from './screens/EditorTimelineScreen';
import { PreviewScreen } from './screens/PreviewScreen';
import { VoiceCaptionScreen } from './screens/VoiceCaptionScreen';
import { RenderQueueScreen } from './screens/RenderQueueScreen';
import { HistoryScreen } from './screens/HistoryScreen';
import { SettingsScreen } from './screens/SettingsScreen';
import { LogsScreen } from './screens/LogsScreen';

export const App: React.FC = () => {
  const [currentScreen, setCurrentScreen] = useState<ScreenId>('dashboard');
  const [currentProject, setCurrentProject] = useState<string | null>('Shorts Review Threads #1');
  const [lang, setLang] = useState<'vi' | 'en'>('vi');

  // Keyboard navigation shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        // command palette shortcut trigger
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const handleOpenProject = (name: string) => {
    setCurrentProject(name);
    setCurrentScreen('editor');
  };

  const renderScreen = () => {
    switch (currentScreen) {
      case 'dashboard':
        return <DashboardScreen onOpenProject={handleOpenProject} />;
      case 'import':
        return <ImportScreen />;
      case 'content':
        return <ContentLibraryScreen />;
      case 'assets':
        return <AssetsLibraryScreen />;
      case 'editor':
        return <EditorTimelineScreen />;
      case 'preview':
        return <PreviewScreen />;
      case 'voice':
        return <VoiceCaptionScreen />;
      case 'render':
        return <RenderQueueScreen />;
      case 'history':
        return <HistoryScreen />;
      case 'settings':
        return (
          <SettingsScreen
            lang={lang}
            onToggleLang={() => setLang(lang === 'vi' ? 'en' : 'vi')}
          />
        );
      case 'logs':
        return <LogsScreen />;
      default:
        return <DashboardScreen onOpenProject={handleOpenProject} />;
    }
  };

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-slate-950 font-sans">
      <Navigation
        currentScreen={currentScreen}
        onSelectScreen={setCurrentScreen}
        lang={lang}
      />
      <div className="flex-1 flex flex-col min-w-0">
        <main className="flex-1 overflow-auto bg-slate-950">{renderScreen()}</main>
        <StatusBar currentProject={currentProject} saveStatus="Đã lưu tự động" />
      </div>
    </div>
  );
};
