'use client';
import React from 'react';

interface VideoPlayerProps {
  url: string;
}

const VideoPlayer: React.FC<VideoPlayerProps> = ({ url }) => {
  return (
    <div className="flex justify-center items-center h-screen">
      <iframe
        src={url}
        title="Video player"
        style={{ width: '80%', height: '80vh', border: 'none' }}
        allowFullScreen
      />
    </div>
  );
};

export default VideoPlayer;
