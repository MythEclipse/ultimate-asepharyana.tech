/**
 * Props for the VideoPlayer component.
 */
interface VideoPlayerProps {
  /** The URL of the video to play (string) */
  url: string;
}

const VideoPlayer: React.FC<VideoPlayerProps> = ({ url }) => {
  const handleButtonClick = () => {
    window.open(url, '_blank');
  };

  return (
    <div className="flex flex-col justify-center items-center h-screen">
      {/* <iframe
        src={url}
        style={{ width: '80%', height: '80vh', border: 'none' }}
        allowFullScreen
      /> */}
      <button
        onClick={handleButtonClick}
        className="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
      >
        Open in New Tab
      </button>
    </div>
  );
};

export default VideoPlayer;
