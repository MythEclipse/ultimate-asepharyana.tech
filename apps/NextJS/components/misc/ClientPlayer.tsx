'use client';
import React, { useState } from 'react';

interface VideoPlayerProps {
  url: string;
}

const VideoPlayer: React.FC<VideoPlayerProps> = ({ url }) => {
  return (
    <div className='flex justify-center items-center h-screen'>
      <iframe
        src={url}
        style={{ width: '80%', height: '80vh', border: 'none' }}
        allowFullScreen
      />
    </div>
  );
};

export default VideoPlayer;
